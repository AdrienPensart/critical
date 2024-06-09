// use futures_lite::future::block_on;
use futures_lite::stream::StreamExt;
use indicatif::{ProgressBar, ProgressStyle};
use rand::{seq::SliceRandom, thread_rng};
use std::collections::HashMap;
use std::num::NonZeroUsize;
use std::path::Path;
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::Arc;
use tokio::sync::Mutex;
use tokio::sync::Semaphore;
use tokio::task::JoinSet;

use crate::errors::CriticalErrorKind;
use crate::helpers::{is_hidden, public_ip};
use crate::music::flac_file::FlacFile;
use crate::music::mp3_file::Mp3File;
use crate::music::BoxMusic;
use crate::queries::{
    HARD_CLEAN_QUERY, UPSERT_ALBUM, UPSERT_ARTIST, UPSERT_FOLDER, UPSERT_GENRE, UPSERT_KEYWORD,
    UPSERT_MUSIC,
};

#[derive(clap::Parser, Debug)]
#[clap(about = "Scan folders and save music")]
pub struct FoldersScanner {
    /// Enable bulk insert / batch
    #[clap(short, long, visible_alias = "batch")]
    pub bulk: bool,

    /// Clean musics before scanning
    #[clap(short, long)]
    pub clean: bool,

    /// Dry insert
    #[clap(long)]
    pub dry: bool,

    #[clap(long)]
    /// EdgeDB DSN
    pub dsn: String,

    // #[clap(long, default_value_t = std::thread::available_parallelism().unwrap())]
    #[clap(long, default_value_t = NonZeroUsize::new(4).unwrap())]
    /// Concurrency
    pub workers: NonZeroUsize,

    #[clap(long, default_value_t = NonZeroUsize::new(3).unwrap())]
    /// Retries in case of failed transaction
    pub retries: NonZeroUsize,

    pub folders: Vec<String>,
}

impl FoldersScanner {
    pub async fn scan(&self) -> Result<(), CriticalErrorKind> {
        let client = edgedb_tokio::Client::new(
            &edgedb_tokio::Builder::new()
                .dsn(&self.dsn)?
                // .client_security(edgedb_tokio::ClientSecurity::InsecureDevMode)
                .build_env()
                .await?,
        );

        if self.clean {
            client.execute(HARD_CLEAN_QUERY, &()).await?;
        }

        let retries: usize = self.retries.into();
        let ipv4 = public_ip().await?;
        let username = whoami::username().to_string();
        let folders: Arc<Mutex<HashMap<String, uuid::Uuid>>> = Arc::new(Mutex::new(HashMap::new()));
        let mut musics: Vec<BoxMusic> = Vec::new();

        for folder in self.folders.iter() {
            let folder_path = Path::new(&folder);
            if !folder_path.is_dir() {
                eprintln!("{folder} : path is not a directory");
                continue;
            }
            let Some(folder_path) = folder_path.to_str() else {
                eprintln!("{folder} : issue with folder path");
                continue;
            };

            let mut folders = folders.lock().await;
            if !folders.contains_key(folder_path) {
                let mut folder_id: Option<uuid::Uuid> = None;
                for _ in 0..retries {
                    let folder_result = client
                        .query_required_single(UPSERT_FOLDER, &(folder_path, &username, &ipv4))
                        .await;
                    match folder_result {
                        Err(e) => {
                            if e.kind_name() != "TransactionSerializationError" {
                                return Err(CriticalErrorKind::EdgedbError(e));
                            }
                            eprintln!("retrying upsert folder {}", folder_path);
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
            }

            // println!("folder_id: {folder_id}");

            let mut walker = async_walkdir::WalkDir::new(folder);
            loop {
                match walker.next().await {
                    Some(Ok(entry)) => {
                        if entry.file_type().await?.is_dir() {
                            continue;
                        }
                        if is_hidden(&entry) {
                            continue;
                        }
                        let entry_path = entry.path();
                        let path = entry_path.to_str();
                        let Some(path) = path else {
                            eprintln!("Issue with path : {:?}", entry.path());
                            continue;
                        };

                        let Some(extension) = entry_path.extension() else {
                            eprintln!("Issue with path extension : {path}");
                            continue;
                        };

                        match extension.to_str() {
                            Some("flac") => {
                                musics.push(Box::new(FlacFile::from_path(folder_path, path)?));
                            }
                            Some("mp3") => {
                                musics.push(Box::new(Mp3File::from_path(folder_path, path)?))
                            }
                            Some("m3u") | Some("jpg") => continue,
                            _ => {
                                eprintln!("Unsupported format : {path}");
                                continue;
                            }
                        };
                    }
                    Some(Err(e)) => {
                        eprintln!("error: {}", e);
                        continue;
                    }
                    None => break,
                }
            }
        }

        {
            // reduces errors among serialization transactions
            let mut rng = thread_rng();
            musics.shuffle(&mut rng);
        }

        let total = musics.len();
        println!("Musics about to be upserted: {total}");
        let bar = ProgressBar::new(total as u64);
        bar.set_style(
            ProgressStyle::default_bar()
                .template("[{elapsed_precise}] {bar:50.cyan/blue} {pos:>7}/{len:7} {msg}")?
                .progress_chars("##-"),
        );
        let bar = Arc::new(Mutex::new(bar));

        let artists: Arc<Mutex<HashMap<String, uuid::Uuid>>> = Arc::new(Mutex::new(HashMap::new()));
        let albums: Arc<Mutex<HashMap<uuid::Uuid, HashMap<String, uuid::Uuid>>>> =
            Arc::new(Mutex::new(HashMap::new()));
        let genres: Arc<Mutex<HashMap<String, uuid::Uuid>>> = Arc::new(Mutex::new(HashMap::new()));
        let keywords: Arc<Mutex<HashMap<String, uuid::Uuid>>> =
            Arc::new(Mutex::new(HashMap::new()));

        let semaphore = Arc::new(Semaphore::new(self.workers.get()));
        let mut set: JoinSet<Result<(uuid::Uuid, String), CriticalErrorKind>> = JoinSet::new();
        let errors = Arc::new(AtomicU64::new(0));

        for music in musics {
            let genres = genres.clone();
            let artists = artists.clone();
            let albums = albums.clone();
            let folders = folders.clone();
            let keywords = keywords.clone();
            let client = client.clone();
            let semaphore = semaphore.clone();
            let errors = errors.clone();
            let bar = bar.clone();

            set.spawn(async move {
                let _permit = semaphore.acquire_owned().await?;

                let artist_id = {
                    let mut artists = artists.lock().await;
                    if !artists.contains_key(music.artist()) {
                        let mut artist_id: Option<uuid::Uuid> = None;
                        for _ in 0..retries {
                            let artist_result = client
                                .query_required_single(UPSERT_ARTIST, &(music.artist(),))
                                .await;
                            match artist_result {
                                Err(e) => {
                                    if e.kind_name() != "TransactionSerializationError" {
                                        return Err(CriticalErrorKind::EdgedbError(e));
                                    }
                                    bar.lock().await.println(format!(
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
                };
                // println!("artist_id: {artist_id}");

                let album_id = {
                    let mut albums = albums.lock().await;
                    if !albums[&artist_id].contains_key(music.album()) {
                        let mut album_id: Option<uuid::Uuid> = None;
                        for _ in 0..retries {
                            let album_result = client
                                .query_required_single(UPSERT_ALBUM, &(music.album(), artist_id))
                                .await;
                            match album_result {
                                Err(e) => {
                                    if e.kind_name() != "TransactionSerializationError" {
                                        return Err(CriticalErrorKind::EdgedbError(e));
                                    }
                                    bar.lock().await.println(format!(
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
                // println!("album_id: {album_id}");

                let genre_id = {
                    let mut genres = genres.lock().await;
                    if !genres.contains_key(music.genre()) {
                        let mut genre_id: Option<uuid::Uuid> = None;
                        for _ in 0..retries {
                            let genre_result = client
                                .query_required_single(UPSERT_GENRE, &(music.genre(),))
                                .await;
                            match genre_result {
                                Err(e) => {
                                    if e.kind_name() != "TransactionSerializationError" {
                                        return Err(CriticalErrorKind::EdgedbError(e));
                                    }
                                    bar.lock().await.println(format!(
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

                let mut keyword_ids = Vec::new();
                {
                    let mut keywords = keywords.lock().await;
                    for keyword in music.keywords().iter() {
                        let keyword_id = if !keywords.contains_key(keyword) {
                            let mut keyword_id: Option<uuid::Uuid> = None;
                            for _ in 0..retries {
                                let keyword_folder = client
                                    .query_required_single(UPSERT_KEYWORD, &(keyword,))
                                    .await;
                                match keyword_folder {
                                    Err(e) => {
                                        if e.kind_name() != "TransactionSerializationError" {
                                            return Err(CriticalErrorKind::EdgedbError(e));
                                        }
                                        bar.lock().await.println(format!(
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
                            keywords[keyword]
                        };
                        keyword_ids.push(keyword_id);
                    }
                }
                // println!("keyword_ids: {keyword_ids:?}");

                let music_id = {
                    let mut music_id: Option<uuid::Uuid> = None;
                    let folders = folders.lock().await;
                    for _ in 0..retries {
                        let rating = music.rating()?;
                        let size = music.size().await?;
                        let music_result = client
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
                                bar.lock()
                                    .await
                                    .println(format!("retrying upsert music {}", music.title()));
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

                Ok((music_id, music.path().to_string()))
            });
        }

        while let Some(result) = set.join_next().await {
            let bar = bar.lock().await;
            match result {
                Ok(Ok((_music_id, _music_path))) => {
                    // bar.println(format!("{music_path} : {music_id}"));
                    bar.inc(1);
                }
                Ok(Err(CriticalErrorKind::UpsertError { object, path })) => {
                    bar.println(format!("error on {path} : {object}"));
                }
                Ok(Err(e)) => {
                    bar.println(format!("error: {:#?}", e));
                }
                Err(e) => bar.println(format!("error: {:#?}", e)),
            };
        }

        let bar = bar.lock().await;
        bar.finish();

        let errors = errors.load(Ordering::Relaxed);
        if errors > 0 {
            eprintln!("Upsert errors: {errors}");
        }

        Ok(())
    }
}
