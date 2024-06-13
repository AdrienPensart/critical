use crate::errors::CriticalErrorKind;
use crate::filter::Filter;
use crate::music::music_result::MusicResult;
use crate::queries::PLAYLIST_QUERY;
use rand::{seq::SliceRandom, thread_rng};
use serde::Serialize;
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

#[derive(clap::Parser, Debug, Default)]
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

    #[clap(long)]
    shuffle: bool,

    #[clap(flatten)]
    filter: Filter,

    // #[clap(long)]
    // interleave: bool,
    out: Option<String>,
}

impl Playlist {
    pub async fn playlist(&self, client: edgedb_tokio::Client) -> Result<(), CriticalErrorKind> {
        if self.filter.min_rating > self.filter.max_rating {
            return Err(CriticalErrorKind::InvalidMinMaxRating {
                min_rating: self.filter.min_rating,
                max_rating: self.filter.max_rating,
            });
        }
        if self.filter.min_length > self.filter.max_length {
            return Err(CriticalErrorKind::InvalidMinMaxLength {
                min_length: self.filter.min_length,
                max_length: self.filter.max_length,
            });
        }
        if self.filter.min_size > self.filter.max_size {
            return Err(CriticalErrorKind::InvalidMinMaxSize {
                min_size: self.filter.min_size,
                max_size: self.filter.max_size,
            });
        }

        let music_filter = serde_json::to_string(&self.filter)?;
        let mut musics: Vec<MusicResult> = client.query(PLAYLIST_QUERY, &(music_filter,)).await?;

        if self.shuffle {
            let mut rng = thread_rng();
            musics.shuffle(&mut rng);
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
