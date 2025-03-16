use std::collections::HashMap;
use std::path::{Path, PathBuf};

use super::albums::Album;
use super::artists::Artist;
use super::cache::UpsertCache;
use super::clean::clean;
use super::config::Config;
use super::errors::CriticalErrorKind;
use super::flac_file::FlacFile;
use super::folders::Folder;
use super::genres::Genre;
use super::helpers::is_hidden;
use super::helpers::{has_unique_elements, public_ip};
use super::keywords::Keyword;
use super::mp3_file::Mp3File;
use super::music::Music;
use super::music_file::BoxMusicFile;
use super::vertex::Vertex;

const DEFAULT_RETRIES: std::num::NonZeroU16 = match std::num::NonZeroU16::new(3) {
    Some(v) => v,
    None => panic!("Bad default value for retries"),
};

#[derive(clap::Parser)]
#[clap(about = "Scan folders and save music")]
pub struct Scan {
    /// Clean musics before scanning
    #[clap(short, long)]
    clean: bool,

    #[clap(long, default_value_t = DEFAULT_RETRIES)]
    /// Retries in case of failed transaction
    retries: std::num::NonZeroU16,

    folders: Vec<String>,
}

impl Scan {
    pub async fn scan(&self, mut config: Config) -> Result<(), CriticalErrorKind> {
        if self.clean {
            Box::pin(clean(&config.gel, false, config.dry)).await?;
        }

        config.retries = self.retries.into();
        Folder::index_indradb(&config.indradb)?;
        Artist::index_indradb(&config.indradb)?;
        Album::index_indradb(&config.indradb)?;
        Genre::index_indradb(&config.indradb)?;
        Keyword::index_indradb(&config.indradb)?;
        Music::index_indradb(&config.indradb)?;

        let ipv4 = public_ip().await?;
        let username = whoami::username().to_string();
        let mut cache = UpsertCache::default();
        let mut count: u64 = 0;
        let mut paths = HashMap::<String, Vec<PathBuf>>::new();
        for folder in &self.folders {
            walkdir::WalkDir::new(Path::new(folder))
                .follow_links(false)
                .into_iter()
                .filter_map(std::result::Result::ok)
                .filter(|e| {
                    !is_hidden(e)
                        && e.file_type().is_file()
                        && (e.file_name().to_string_lossy().ends_with(".flac")
                            || e.file_name().to_string_lossy().ends_with(".mp3"))
                })
                .for_each(|entry| {
                    paths
                        .entry(folder.clone())
                        .or_default()
                        .push(entry.path().to_path_buf());
                    count += 1;
                });
        }

        let load_music_files_bar = indicatif::ProgressBar::new(count);
        load_music_files_bar.set_style(
            indicatif::ProgressStyle::default_bar()
                .template(
                    "[{elapsed_precise}] Loading files: {bar:50.cyan/blue} {pos:>7}/{len:7} {msg}",
                )?
                .progress_chars("##-"),
        );

        for (folder, paths) in &paths {
            let folder_path = Path::new(&folder);
            let Some(folder_path) = folder_path.to_str() else {
                load_music_files_bar.println(format!("{folder} : issue with folder path"));
                continue;
            };

            let folder = Folder {
                name: folder.clone(),
                ipv4: ipv4.clone(),
                username: username.clone(),
            };

            let folder_indradb = folder.upsert_indradb(&config)?;
            let folder_gel = folder.upsert_gel(&config, &mut cache).await?;

            for path in paths {
                scopeguard::defer! {load_music_files_bar.inc(1)};

                let Some(extension) = path.extension() else {
                    load_music_files_bar.println(format!(
                        "Issue with path extension : {}",
                        path.to_string_lossy()
                    ));
                    continue;
                };

                let path_str = path.to_string_lossy().to_string();
                let music: BoxMusicFile = match extension.to_str() {
                    Some("flac") => Box::new(FlacFile::from_path(folder_path, &path_str)?),
                    Some("mp3") => Box::new(Mp3File::from_path(folder_path, &path_str)?),
                    Some("m3u" | "jpg") => continue,
                    _ => {
                        load_music_files_bar.println(format!("Unsupported format : {}", &path_str));
                        continue;
                    }
                };

                if !has_unique_elements(music.keywords()) {
                    load_music_files_bar.println(format!(
                        "Music has duplicated keywords : {} / {}",
                        &path_str,
                        music.keywords().join(", ")
                    ));
                    continue;
                }

                let artist = Artist {
                    name: music.artist().to_string(),
                };
                let artist_indradb = artist.upsert_indradb(&config)?;
                let artist_gel = artist.upsert_gel(&config, &mut cache).await?;

                let album = Album {
                    name: music.album().to_string(),
                    artist_indradb,
                    artist_gel,
                };
                let album_indradb = album.upsert_indradb(&config)?;
                let album_gel = album.upsert_gel(&config, &mut cache).await?;

                let genre = Genre {
                    name: music.genre().to_string(),
                };
                let genre_indradb = genre.upsert_indradb(&config)?;
                let genre_gel = genre.upsert_gel(&config, &mut cache).await?;

                let mut keywords_indradb = Vec::new();
                let mut keywords_gel = Vec::new();
                {
                    for keyword in music.keywords() {
                        let keyword = Keyword { name: keyword };
                        let keyword_indradb = keyword.upsert_indradb(&config)?;
                        keywords_indradb.push(keyword_indradb);
                        let keyword_gel = keyword.upsert_gel(&config, &mut cache).await?;
                        keywords_gel.push(keyword_gel);
                    }
                }

                let music = Music {
                    track: music.track(),
                    title: music.title().to_string(),
                    rating: music.rating()?,
                    size: i64::try_from(music.size().await?)?,
                    length: music.length(),
                    path: music.path().to_string(),

                    keywords_indradb,
                    keywords_gel,

                    genre_indradb,
                    genre_gel,

                    folder_indradb,
                    folder_gel,

                    artist_indradb,
                    artist_gel,

                    album_indradb,
                    album_gel,
                };
                let _music_indradb = music.upsert_indradb(&config);
                let _music_gel = music.upsert_gel(&config, &mut cache).await?;
            }
        }
        load_music_files_bar.finish();

        if !config.dry && !config.no_indradb {
            config.indradb.sync()?;
        }
        Ok(())
    }
}
