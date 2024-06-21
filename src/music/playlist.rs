use crate::music::errors::CriticalErrorKind;
use crate::music::filter::Filter;
use crate::music::helpers::interleave_evenly;
use crate::music::music_result::MusicResult;
use crate::music::queries::PLAYLIST_QUERY;
use rand::{seq::SliceRandom, thread_rng};
use serde::Serialize;
use std::collections::{HashMap, HashSet};
use tabled::Table;
const DEFAULT_NAME: &str = "default";

#[derive(clap::ValueEnum, Clone, Default, Debug, Serialize, PartialEq)]
#[serde(rename_all = "kebab-case")]
enum Output {
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
pub struct Playlist {
    #[clap(long, default_value_t = DEFAULT_NAME.to_string())]
    name: String,

    #[clap(long, default_value_t, value_enum)]
    output: Output,

    #[clap(long, value_enum)]
    kind: Vec<Kind>,

    #[clap(long)]
    relative: bool,

    #[clap(flatten)]
    filter: Filter,

    #[clap(name = "filter", long, value_parser = validate_filters)]
    filters: Vec<Filter>,

    #[clap(long, group = "order")]
    interleave: bool,

    #[clap(long, group = "order")]
    shuffle: bool,

    out: Option<String>,
}

fn validate_filters(filter: &str) -> Result<Filter, String> {
    match serde_keyvalue::from_key_values::<Filter>(filter) {
        Ok(filter) => {
            if filter.min_rating > filter.max_rating {
                return Err(CriticalErrorKind::InvalidMinMaxRating {
                    min_rating: filter.min_rating,
                    max_rating: filter.max_rating,
                }
                .to_string());
            }
            if filter.min_length > filter.max_length {
                return Err(CriticalErrorKind::InvalidMinMaxLength {
                    min_length: filter.min_length,
                    max_length: filter.max_length,
                }
                .to_string());
            }
            if filter.min_size > filter.max_size {
                return Err(CriticalErrorKind::InvalidMinMaxSize {
                    min_size: filter.min_size,
                    max_size: filter.max_size,
                }
                .to_string());
            }
            Ok(filter)
        }
        Err(e) => Err(e.to_string()),
    }
}

impl Playlist {
    pub async fn playlist(&self, client: edgedb_tokio::Client) -> Result<(), CriticalErrorKind> {
        let mut musics: HashSet<MusicResult> = HashSet::new();

        let mut filters = self.filters.clone();
        if filters.is_empty() || self.filter != Filter::default() {
            filters.push(self.filter.clone());
        }
        for filter in &filters {
            let music_filter = serde_json::to_string(filter)?;
            let music_results: Vec<MusicResult> =
                client.query(PLAYLIST_QUERY, &(music_filter,)).await?;
            musics.extend(music_results);
        }

        let mut musics: Vec<_> = musics.into_iter().collect();

        if self.shuffle {
            let mut rng = thread_rng();
            musics.shuffle(&mut rng);
        }

        if self.interleave {
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

        let kind = if self.kind.is_empty() {
            vec![Kind::Local]
        } else {
            self.kind.clone()
        };

        if musics.is_empty() && self.output != Output::Json {
            return Ok(());
        }

        let playlist = match self.output {
            Output::M3u => {
                let mut playlist = "#EXTM3U\n".to_string();

                playlist.push_str(&format!("#EXTREM:{}\n", self.name));

                let mut links = Vec::new();
                for music in musics {
                    links.extend_from_slice(music.all_links(self.relative, &kind)?.as_slice());
                }

                let links = links.join("\n");
                playlist.push_str(&links);
                playlist
            }
            Output::Table => Table::new(musics).to_string(),
            Output::Json => serde_json::to_string_pretty(&musics)?,
        };
        if let Some(out) = &self.out {
            std::fs::write(out, playlist)?;
        } else {
            print!("{playlist}");
        };
        Ok(())
    }
}
