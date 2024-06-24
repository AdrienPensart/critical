use crate::music::errors::CriticalErrorKind;
use crate::music::filter::Filters;
use crate::music::helpers::interleave_evenly;
use crate::music::music_result::MusicResult;
use crate::music::MUSIC_FIELDS;
use const_format::concatcp;
use edgedb_derive::Queryable;
use rand::{seq::SliceRandom, thread_rng};
use serde::Serialize;
use std::collections::{HashMap, HashSet};
use tabled::Table;
const DEFAULT_NAME: &str = "default";

#[derive(clap::ValueEnum, Clone, Default, Debug, Serialize, PartialEq)]
#[serde(rename_all = "kebab-case")]
pub enum Output {
    #[default]
    M3u,
    Json,
    Table,
}

#[derive(clap::ValueEnum, Clone, Debug, Serialize, PartialEq)]
#[serde(rename_all = "kebab-case")]
pub enum Kind {
    Local,
    Remote,
    RemoteSSH,
    LocalSSH,
    LocalHTTP,
    RemoteHttp,
    All,
}

#[derive(clap::Parser, Default)]
#[clap(about = "Create playlist")]
pub struct PlaylistCommand {
    /// Playlist name
    #[clap(long, default_value_t = DEFAULT_NAME.to_string())]
    name: String,

    /// Playlist options
    #[clap(flatten)]
    playlist_options: PlaylistOptions,

    /// Output options
    #[clap(flatten)]
    output_options: OutputOptions,

    /// More filters
    #[clap(flatten)]
    filters: Filters,
}

#[derive(clap::Parser, Default, Clone)]
pub struct OutputOptions {
    /// Output format
    #[clap(long, default_value_t, value_enum)]
    output: Output,

    /// Optional output path
    #[clap(long)]
    out: Option<String>,
}

impl OutputOptions {
    pub fn new(output: &Output, out: &Option<String>) -> Self {
        Self {
            output: output.clone(),
            out: out.clone(),
        }
    }
    pub fn out(&self) -> &Option<String> {
        &self.out
    }
    pub fn output(&self) -> &Output {
        &self.output
    }
}

#[derive(clap::Parser, Default)]
pub struct PlaylistOptions {
    #[clap(long, value_enum)]
    kind: Vec<Kind>,

    #[clap(long)]
    relative: bool,

    #[clap(long, group = "order")]
    interleave: bool,

    #[clap(long, group = "order")]
    shuffle: bool,
}

#[derive(Queryable, Clone)]
pub struct Playlist {
    name: String,
    musics: Vec<MusicResult>,
}

impl Playlist {
    pub fn new(name: &str, musics: &[MusicResult]) -> Self {
        Self {
            name: name.to_string(),
            musics: musics.to_vec(),
        }
    }
    pub fn name(&self) -> &str {
        &self.name
    }
    pub fn len(&self) -> usize {
        self.musics.len()
    }
    pub fn is_empty(&self) -> bool {
        self.musics.is_empty()
    }
    pub fn generate(
        &self,
        output_options: &OutputOptions,
        playlist_options: &PlaylistOptions,
        dry: bool,
    ) -> Result<(), CriticalErrorKind> {
        let mut musics = self.musics.clone();
        if playlist_options.shuffle {
            let mut rng = thread_rng();
            musics.shuffle(&mut rng);
        }

        if playlist_options.interleave {
            let mut artist_to_musics: HashMap<String, Vec<MusicResult>> = HashMap::new();
            for music in musics {
                let artist_name = music.artist_name.clone();
                let amusics = artist_to_musics.entry(artist_name).or_default();
                amusics.push(music);
            }
            let values = artist_to_musics
                .into_values()
                .collect::<Vec<Vec<MusicResult>>>();
            musics = interleave_evenly(values);
        }

        let kind = if playlist_options.kind.is_empty() {
            vec![Kind::Local]
        } else {
            playlist_options.kind.clone()
        };

        if musics.is_empty() && output_options.output != Output::Json {
            return Ok(());
        }

        let playlist = match output_options.output {
            Output::M3u => {
                let mut playlist = "#EXTM3U\n".to_string();

                playlist.push_str(&format!("#EXTREM:name={}\n", self.name));
                if let Some(out) = &output_options.out {
                    playlist.push_str(&format!("#EXTREM:path={}\n", out));
                }

                let mut links = Vec::new();
                for music in musics {
                    links.extend_from_slice(
                        music
                            .all_links(playlist_options.relative, &kind)?
                            .as_slice(),
                    );
                }

                let links = links.join("\n");
                playlist.push_str(&links);
                playlist
            }
            Output::Table => Table::new(musics).to_string(),
            Output::Json => serde_json::to_string_pretty(&musics)?,
        };
        if !dry {
            if let Some(out) = &output_options.out {
                std::fs::write(out, playlist)?;
                return Ok(());
            }
        }
        print!("{playlist}");
        Ok(())
    }
}

impl PlaylistCommand {
    pub async fn playlist(
        &self,
        client: edgedb_tokio::Client,
        dry: bool,
    ) -> Result<(), CriticalErrorKind> {
        let mut musics: HashSet<MusicResult> = HashSet::new();

        for filter in &self.filters.all() {
            let music_filter = serde_json::to_string(filter)?;
            let music_results: Vec<MusicResult> =
                client.query(PLAYLIST_QUERY, &(music_filter,)).await?;
            musics.extend(music_results);
        }

        let musics = musics.into_iter().collect::<Vec<MusicResult>>();

        let playlist = Playlist::new(&self.name, &musics);
        playlist.generate(&self.output_options, &self.playlist_options, dry)?;

        Ok(())
    }
}

pub const PLAYLIST_QUERY: &str = concatcp!(
    r#"
    with music_filter := to_json(<str>$0),
    select Music {
        "#,
    MUSIC_FIELDS,
    r#"
    }
    filter
        .length >= <Length>music_filter['min_length'] and .length <= <Length>music_filter['max_length']
        and .size >= <Size>music_filter['min_size'] and .size <= <Size>music_filter['max_size']
        and .rating >= <Rating>music_filter['min_rating'] and .rating <= <Rating>music_filter['max_rating']
        and re_test(<str>music_filter['artist'], .artist.name)
        and re_test(<str>music_filter['album'], .album.name)
        and re_test(<str>music_filter['genre'], .genre.name)
        and re_test(<str>music_filter['title'], .name)
        and re_test(<str>music_filter['keyword'], array_join(array_agg((select .keywords.name)), " "))
        and (<str>music_filter['pattern'] = "" or ext::pg_trgm::word_similar(<str>music_filter['pattern'], .title))
    order by
        .artist.name then
        .album.name then
        .track then
        .name
    limit <`Limit`>music_filter['limit']
"#
);
