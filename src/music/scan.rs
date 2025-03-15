use async_walkdir::Filtering;
use futures_lite::stream::StreamExt;
use std::collections::HashMap;
// use indradb::ijson;
// use indradb::Query;
// use indradb::VertexWithPropertyValueQuery;
// use indradb::{BulkInsertItem, Vertex};
// use serde::Serialize;
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::Arc;
use tokio::sync::Mutex;

use crate::commands::opts::Config;
use crate::music::clean::clean;
use crate::music::errors::CriticalErrorKind;
use crate::music::flac_file::FlacFile;
use crate::music::folders::FolderVertex;
use crate::music::helpers::public_ip;
use crate::music::keywords::KeywordVertex;
use crate::music::mp3_file::Mp3File;
use crate::music::music_input::MusicInput;
use crate::music::{Music, MusicVertex};

use super::albums::AlbumVertex;
use super::artists::ArtistVertex;
use super::genres::GenreVertex;
use super::helpers::{async_is_hidden, is_hidden};

pub type BoxMusic = Box<dyn Music + Send + Sync>;

const DEFAULT_WORKERS: std::num::NonZeroUsize = match std::num::NonZeroUsize::new(4) {
    Some(v) => v,
    None => panic!("Bad default value for workers"),
};

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

    // #[clap(long, default_value_t = std::thread::available_parallelism().unwrap())]
    #[clap(long, default_value_t = DEFAULT_WORKERS)]
    /// Concurrency
    workers: std::num::NonZeroUsize,

    #[clap(long, default_value_t = DEFAULT_RETRIES)]
    /// Retries in case of failed transaction
    retries: std::num::NonZeroU16,

    folders: Vec<String>,
}

impl Scan {
    pub async fn scan(&self, config: Config) -> Result<(), CriticalErrorKind> {
        if self.clean {
            clean(&config.client, false, config.dry).await?;
        }

        let db = if config.no_indradb {
            indradb::MemoryDatastore::new_db()
        } else if config.datastore.exists() {
            indradb::MemoryDatastore::read_msgpack_db(config.datastore)?
        } else {
            indradb::MemoryDatastore::create_msgpack_db(config.datastore)
        };

        FolderVertex::index(&db)?;
        ArtistVertex::index(&db)?;
        AlbumVertex::index(&db)?;
        GenreVertex::index(&db)?;
        KeywordVertex::index(&db)?;
        MusicVertex::index(&db)?;

        let ipv4 = public_ip().await?;
        let username = whoami::username().to_string();
        let retries: u16 = self.retries.into();
        let errors = Arc::new(AtomicU64::new(0));
        let folders: Arc<Mutex<HashMap<String, uuid::Uuid>>> = Arc::new(Mutex::new(HashMap::new()));
        let artists: Arc<Mutex<HashMap<String, uuid::Uuid>>> = Arc::new(Mutex::new(HashMap::new()));
        let albums: Arc<Mutex<HashMap<uuid::Uuid, HashMap<String, uuid::Uuid>>>> =
            Arc::new(Mutex::new(HashMap::new()));
        let genres: Arc<Mutex<HashMap<String, uuid::Uuid>>> = Arc::new(Mutex::new(HashMap::new()));
        let keywords: Arc<Mutex<HashMap<String, uuid::Uuid>>> =
            Arc::new(Mutex::new(HashMap::new()));

        let mut count: u64 = 0;
        let mut paths = HashMap::<String, Vec<PathBuf>>::new();
        for folder in self.folders.iter() {
            walkdir::WalkDir::new(Path::new(folder))
                .follow_links(false)
                .into_iter()
                .filter_map(|entry| entry.ok())
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

        let mut music_inputs = Vec::new();
        for folder in self.folders.iter() {
            let folder_path = Path::new(&folder);
            let Some(folder_path) = folder_path.to_str() else {
                load_music_files_bar.println(format!("{folder} : issue with folder path"));
                continue;
            };

            let mut folders = folders.lock().await;
            let _folder_id = if !folders.contains_key(folder_path) {
                let mut folder_id: Option<uuid::Uuid> = None;
                for _ in 0..retries {
                    let folder_result = config
                        .client
                        .query_required_single(UPSERT_FOLDER, &(folder_path, &username, &ipv4))
                        .await;
                    match folder_result {
                        Err(e) => {
                            if e.kind_name() != "TransactionSerializationError" {
                                return Err(CriticalErrorKind::EdgedbError(e));
                            }
                            load_music_files_bar
                                .println(format!("retrying upsert folder {folder_path}"));
                            errors.fetch_add(1, Ordering::Relaxed);
                        }
                        Ok(id) => {
                            folder_id = Some(id);
                            break;
                        }
                    };
                }
                let Some(folder_id) = folder_id else {
                    return Err(CriticalErrorKind::UpsertError {
                        path: folder_path.to_string(),
                        object: folder_path.to_string(),
                    });
                };
                folders.insert(folder_path.to_string(), folder_id);
                folder_id
            } else {
                folders[folder_path]
            };

            let mut walker = async_walkdir::WalkDir::new(folder).filter(|e| async move {
                if e.file_type().await.unwrap().is_file() && !async_is_hidden(&e) {
                    Filtering::Continue
                } else {
                    Filtering::Ignore
                }
            });
            loop {
                scopeguard::defer! {load_music_files_bar.inc(1)};

                match walker.next().await {
                    Some(Ok(entry)) => {
                        let entry_path = entry.path();
                        let path = entry_path.to_str();
                        let Some(path) = path else {
                            load_music_files_bar
                                .println(format!("Issue with path : {:?}", entry.path()));
                            continue;
                        };

                        let Some(extension) = entry_path.extension() else {
                            load_music_files_bar
                                .println(format!("Issue with path extension : {path}"));
                            continue;
                        };

                        let music: BoxMusic = match extension.to_str() {
                            Some("flac") => Box::new(FlacFile::from_path(folder_path, path)?),
                            Some("mp3") => Box::new(Mp3File::from_path(folder_path, path)?),
                            Some("m3u") | Some("jpg") => continue,
                            _ => {
                                load_music_files_bar
                                    .println(format!("Unsupported format : {path}"));
                                continue;
                            }
                        };

                        if !config.no_indradb {
                            let music_input = MusicInput {
                                title: music.title().to_string(),
                                artist: music.artist().to_string(),
                                album: music.album().to_string(),
                                genre: music.genre().to_string(),
                                length: music.length(),
                                size: music.size().await?,
                                track: music.track(),
                                rating: music.rating()?,
                                keywords: music.keywords().clone(),
                                folder: folder_path.to_string(),
                                path: path.to_string(),
                                ipv4: ipv4.clone(),
                                username: username.clone(),
                            };
                            music_inputs.push(music_input);
                        }

                        if config.no_gel {
                            continue;
                        }

                        let artist_id = {
                            let mut artists = artists.lock().await;
                            if !artists.contains_key(music.artist()) {
                                let mut artist_id: Option<uuid::Uuid> = None;
                                for _ in 0..retries {
                                    let artist_result = config
                                        .client
                                        .query_required_single(UPSERT_ARTIST, &(music.artist(),))
                                        .await;
                                    match artist_result {
                                        Err(e) => {
                                            if e.kind_name() != "TransactionSerializationError" {
                                                return Err(CriticalErrorKind::EdgedbError(e));
                                            }
                                            load_music_files_bar.println(format!(
                                                "retrying upsert artist {}",
                                                music.artist()
                                            ));
                                            errors.fetch_add(1, Ordering::Relaxed);
                                        }
                                        Ok(id) => {
                                            artist_id = Some(id);
                                            break;
                                        }
                                    };
                                }
                                let Some(artist_id) = artist_id else {
                                    return Err(CriticalErrorKind::UpsertError {
                                        path: music.path().to_string(),
                                        object: music.artist().to_string(),
                                    });
                                };
                                artists.insert(music.artist().to_string(), artist_id);
                                {
                                    let mut albums = albums.lock().await;
                                    albums.insert(artist_id, HashMap::new());
                                }
                                artist_id
                            } else {
                                artists[music.artist()]
                            }
                            // println!("artist_id: {artist_id}");
                        };

                        let album_id = {
                            let mut albums = albums.lock().await;
                            if !albums[&artist_id].contains_key(music.album()) {
                                let mut album_id: Option<uuid::Uuid> = None;
                                for _ in 0..retries {
                                    let album_result = config
                                        .client
                                        .query_required_single(
                                            UPSERT_ALBUM,
                                            &(music.album(), artist_id),
                                        )
                                        .await;
                                    match album_result {
                                        Err(e) => {
                                            if e.kind_name() != "TransactionSerializationError" {
                                                return Err(CriticalErrorKind::EdgedbError(e));
                                            }
                                            load_music_files_bar.println(format!(
                                                "retrying upsert album {}",
                                                music.album()
                                            ));
                                            errors.fetch_add(1, Ordering::Relaxed);
                                        }
                                        Ok(id) => {
                                            album_id = Some(id);
                                            break;
                                        }
                                    };
                                }
                                let Some(album_id) = album_id else {
                                    return Err(CriticalErrorKind::UpsertError {
                                        path: music.path().to_string(),
                                        object: music.album().to_string(),
                                    });
                                };
                                if let Some(albums) = albums.get_mut(&artist_id) {
                                    albums.insert(music.album().to_string(), album_id);
                                }
                                album_id
                            } else {
                                albums[&artist_id][music.album()]
                            }
                        };

                        // println!("album_vertex_id: {album_vertex_id}");
                        // println!("album_id: {album_id}");

                        let genre_id = {
                            let mut genres = genres.lock().await;
                            if !genres.contains_key(music.genre()) {
                                let mut genre_id: Option<uuid::Uuid> = None;
                                for _ in 0..retries {
                                    let genre_result = config
                                        .client
                                        .query_required_single(UPSERT_GENRE, &(music.genre(),))
                                        .await;
                                    match genre_result {
                                        Err(e) => {
                                            if e.kind_name() != "TransactionSerializationError" {
                                                return Err(CriticalErrorKind::EdgedbError(e));
                                            }
                                            load_music_files_bar.println(format!(
                                                "retrying upsert genre {}",
                                                music.genre()
                                            ));
                                            errors.fetch_add(1, Ordering::Relaxed);
                                        }
                                        Ok(id) => {
                                            genre_id = Some(id);
                                            break;
                                        }
                                    };
                                }
                                let Some(genre_id) = genre_id else {
                                    return Err(CriticalErrorKind::UpsertError {
                                        path: music.path().to_string(),
                                        object: music.genre().to_string(),
                                    });
                                };
                                genres.insert(music.genre().to_string(), genre_id);
                                genre_id
                            } else {
                                genres[music.genre()]
                            }
                        };
                        // println!("genre_id: {genre_id}");

                        let mut keywords = keywords.lock().await;
                        let mut keyword_ids = Vec::new();
                        {
                            for keyword in music.keywords() {
                                let keyword_id = if !keywords.contains_key(&keyword) {
                                    let mut keyword_id: Option<uuid::Uuid> = None;
                                    for _ in 0..retries {
                                        let keyword_folder = config
                                            .client
                                            .query_required_single(UPSERT_KEYWORD, &(&keyword,))
                                            .await;
                                        match keyword_folder {
                                            Err(e) => {
                                                if e.kind_name() != "TransactionSerializationError"
                                                {
                                                    return Err(CriticalErrorKind::EdgedbError(e));
                                                }
                                                load_music_files_bar.println(format!(
                                                    "retrying upsert keyword {}",
                                                    keyword
                                                ));
                                                errors.fetch_add(1, Ordering::Relaxed);
                                            }
                                            Ok(id) => {
                                                keyword_id = Some(id);
                                                break;
                                            }
                                        };
                                    }
                                    let Some(keyword_id) = keyword_id else {
                                        return Err(CriticalErrorKind::UpsertError {
                                            path: music.path().to_string(),
                                            object: keyword.clone(),
                                        });
                                    };
                                    keywords.insert(keyword.to_string(), keyword_id);
                                    keyword_id
                                } else {
                                    keywords[&keyword]
                                };
                                keyword_ids.push(keyword_id);
                            }
                        }
                        // println!("keyword_ids: {keyword_ids:?}");

                        let _music_id = {
                            let mut music_id: Option<uuid::Uuid> = None;
                            // let folders = folders.lock().await;
                            for _ in 0..retries {
                                let rating: f64 = music.rating()?.into();
                                let size = music.size().await?;
                                let music_result = config
                                    .client
                                    .query_required_single(
                                        UPSERT_MUSIC,
                                        &(
                                            music.title(),
                                            album_id,
                                            genre_id,
                                            size as i64,
                                            music.length(),
                                            keyword_ids.clone(),
                                            music.track(),
                                            rating,
                                            folders[music.folder()],
                                            music.path(),
                                        ),
                                    )
                                    .await;
                                match music_result {
                                    Err(e) => {
                                        if e.kind_name() != "TransactionSerializationError" {
                                            return Err(CriticalErrorKind::EdgedbError(e));
                                        }
                                        load_music_files_bar.println(format!(
                                            "retrying upsert music {}",
                                            music.title()
                                        ));
                                        errors.fetch_add(1, Ordering::Relaxed);
                                    }
                                    Ok(id) => {
                                        music_id = Some(id);
                                        break;
                                    }
                                };
                            }
                            let Some(music_id) = music_id else {
                                return Err(CriticalErrorKind::UpsertError {
                                    path: music.path().to_string(),
                                    object: music.title().to_string(),
                                });
                            };
                            music_id
                        };
                    }
                    Some(Err(e)) => {
                        load_music_files_bar.println(format!("error: {}", e));
                        continue;
                    }
                    None => break,
                }
            }
        }
        load_music_files_bar.finish();

        if !config.dry && !config.no_indradb {
            db.sync()?;
        }

        let errors = errors.load(Ordering::Relaxed);
        if errors > 0 {
            eprintln!("Upsert errors: {errors}");
        }

        Ok(())
    }
}

const UPSERT_FOLDER: &str = r#"
select upsert_folder(
    folder := <str>$0,
    username := <str>$1,
    ipv4 := <str>$2
).id
"#;

const UPSERT_ARTIST: &str = r#"
select upsert_artist(
    artist := <str>$0
).id
"#;

const UPSERT_ALBUM: &str = r#"
select upsert_album(
    artist := <Artist>$1,
    album := <str>$0
).id
"#;

const UPSERT_GENRE: &str = r#"
select upsert_genre(
    genre := <str>$0
).id
"#;

const UPSERT_KEYWORD: &str = r#"
select upsert_keyword(
    keyword := <str>$0
).id
"#;

const UPSERT_MUSIC: &str = r#"
select upsert_music(
    title := <str>$0,
    size := <Size>$3,
    length := <Length>$4,
    genre := <Genre>$2,
    album := <Album>$1,
    keywords := <array<uuid>>$5,
    track := <optional Track>$6,
    rating := <Rating>$7,
    folder := <Folder>$8,
    path := <str>$9
).id
"#;
