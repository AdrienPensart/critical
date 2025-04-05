use const_format::concatcp;
use gel_derive::Queryable;
use rand::{rng, seq::SliceRandom};
use serde::Serialize;
use std::collections::{HashMap, HashSet};
use std::fmt::Write;
use tabled::Table;

use super::errors::CriticalErrorKind;
use super::filter::Filters;
use super::helpers::interleave_evenly;
use super::music::MUSIC_FIELDS;
use super::music_result::MusicResult;

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
    #[must_use]
    pub fn new(output: &Output, out: &Option<String>) -> Self {
        Self {
            output: output.clone(),
            out: out.clone(),
        }
    }
    #[must_use]
    pub fn out(&self) -> &Option<String> {
        &self.out
    }
    #[must_use]
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
    #[must_use]
    pub fn new(name: &str, musics: &[MusicResult]) -> Self {
        Self {
            name: name.to_string(),
            musics: musics.to_vec(),
        }
    }
    #[must_use]
    pub fn name(&self) -> &str {
        &self.name
    }
    #[must_use]
    pub fn len(&self) -> usize {
        self.musics.len()
    }
    #[must_use]
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
            let mut rng = rng();
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
            musics = interleave_evenly(values)?;
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

                writeln!(playlist, "#EXTREM:name={}", self.name)?;
                if let Some(out) = &output_options.out {
                    writeln!(playlist, "#EXTREM:path={out}")?;
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
    pub async fn playlist(&self, client: gel_tokio::Client) -> Result<Playlist, CriticalErrorKind> {
        let mut musics: HashSet<MusicResult> = HashSet::new();
        for filter in &self.filters.all() {
            let music_filter = serde_json::to_string(filter)?;
            let music_results: Vec<MusicResult> =
                Box::pin(client.query(PLAYLIST_QUERY, &(music_filter,))).await?;
            musics.extend(music_results);
        }
        let musics = musics.into_iter().collect::<Vec<MusicResult>>();
        Ok(Playlist::new(&self.name, &musics))
    }
    #[must_use]
    pub fn output_options(&self) -> &OutputOptions {
        &self.output_options
    }
    #[must_use]
    pub fn playlist_options(&self) -> &PlaylistOptions {
        &self.playlist_options
    }
}

pub const PLAYLIST_QUERY: &str = concatcp!(
    "
    with music_filter := to_json(<str>$0),
    select gen_playlist(
        min_length := <Length>music_filter['min_length'],
        max_length := <Length>music_filter['max_length'],
        min_size := <Size>music_filter['min_size'],
        max_size := <Size>music_filter['max_size'],
        min_rating := <Rating>music_filter['min_rating'],
        max_rating := <Rating>music_filter['max_rating'],
        artist := <str>music_filter['artist'],
        album := <str>music_filter['album'],
        genre := <str>music_filter['genre'],
        title := <str>music_filter['title'],
        keyword := <str>music_filter['keyword'],
        pattern := <str>music_filter['pattern'],
        limit := <`Limit`>music_filter['limit']
    ) {
        ",
    MUSIC_FIELDS,
    r"
    }
"
);
