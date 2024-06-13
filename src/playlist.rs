use crate::errors::CriticalErrorKind;
use crate::music::music_result::MusicResult;
use crate::queries::PLAYLIST_QUERY;
use rand::{seq::SliceRandom, thread_rng};
use serde::Serialize;
use tabled::Table;

const MATCH_ALL: &str = "(.*?)";
const DEFAULT_PATTERN: &str = "";

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

#[derive(clap::Parser, Debug, Serialize)]
#[clap(about = "Create playlist")]
pub struct Playlist {
    #[clap(long, default_value_t = 0)]
    min_length: i64,
    #[clap(long, default_value_t = i64::MAX)]
    max_length: i64,
    #[clap(long, default_value_t = 0)]
    min_size: i64,
    #[clap(long, default_value_t = i64::MAX)]
    max_size: i64,
    #[clap(long, default_value_t = 0.0)]
    min_rating: f64,
    #[clap(long, default_value_t = 5.0)]
    max_rating: f64,
    #[clap(long, default_value_t = MATCH_ALL.to_string())]
    artist: String,
    #[clap(long, default_value_t = MATCH_ALL.to_string())]
    album: String,
    #[clap(long, default_value_t = MATCH_ALL.to_string())]
    genre: String,
    #[clap(long, default_value_t = MATCH_ALL.to_string())]
    title: String,
    #[clap(long, default_value_t = MATCH_ALL.to_string())]
    keyword: String,
    #[clap(long, default_value_t = DEFAULT_PATTERN.to_string())]
    pattern: String,
    #[clap(long, default_value_t = i64::MAX)]
    limit: i64,

    #[serde(skip)]
    #[clap(long)]
    name: Option<String>,

    #[serde(skip)]
    #[clap(long, default_value_t, value_enum)]
    output: Output,

    #[serde(skip)]
    #[clap(long, value_enum)]
    kind: Vec<Kind>,

    #[serde(skip)]
    #[clap(long)]
    relative: bool,

    #[serde(skip)]
    #[clap(long)]
    shuffle: bool,

    // #[serde(skip)]
    // #[clap(long)]
    // interleave: bool,
    out: Option<String>,
}

impl Playlist {
    pub async fn playlist(&self, dsn: String) -> Result<(), CriticalErrorKind> {
        let client = edgedb_tokio::Client::new(
            &edgedb_tokio::Builder::new()
                .dsn(&dsn)?
                // .client_security(edgedb_tokio::ClientSecurity::InsecureDevMode)
                .build_env()
                .await?,
        );

        let music_filter = serde_json::to_string(&self)?;
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
                if let Some(name) = self.name.clone() {
                    playlist.push_str(&format!("#EXTREM:{}\n", name));
                }

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
