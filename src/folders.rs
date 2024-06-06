use clap::Parser;
use edgedb_tokio::create_client;
use edgedb_tokio::Client;
use indicatif::{ProgressBar, ProgressStyle};
use std::path::Path;
use walkdir::WalkDir;
use whoami::username;

use crate::errors::CriticalErrorKind;
use crate::helpers::{is_hidden, public_ip};
use crate::music::flac_file::FlacFile;
use crate::music::mp3_file::Mp3File;
use crate::music::Music;

#[derive(Parser, Debug)]
#[clap(about = "Scan folders and save music")]
pub struct FoldersScanner {
    /// Enable bulk insert / batch
    #[clap(short, long, visible_alias = "batch")]
    pub bulk: bool,

    /// Clean musics before scanning
    #[clap(short, long)]
    pub clean: bool,

    /// Dry insert
    #[clap(short, long)]
    pub dry: bool,

    pub folders: Vec<String>,
}

async fn upsert_musics(
    musics: &Vec<Box<dyn Music + Send + Sync>>,
    conn: &Client,
    bar: &ProgressBar,
    username: &str,
    ipv4: &str,
    dry: bool,
) {
    for music in musics {
        if dry {
            bar.println(format!("(dry) inserting {music}"));
        } else {
            let music_output = music.upsert(conn, ipv4, username).await;
            match music_output {
                Ok(music_output) => {
                    let id = music_output.id;
                    let name = music_output.name;
                    bar.println(format!("{id} : {name}"));
                }
                Err(e) => bar.println(format!("{}, error: {:#?}", music.path(), e)),
            };
            bar.inc(1);
        }
    }
}

impl FoldersScanner {
    pub async fn scan(&self) -> Result<(), CriticalErrorKind> {
        let conn = create_client().await?;

        let mut musics: Vec<Box<dyn Music + Send + Sync>> = Vec::new();
        for folder in self.folders.iter() {
            let folder_path = Path::new(&folder);
            if !folder_path.is_dir() {
                eprintln!("{} : path is not a directory", folder);
                continue;
            }
            let walker = WalkDir::new(folder).into_iter();
            for entry in walker.filter_entry(|e| !is_hidden(e)).flatten() {
                if !entry.file_type().is_file() {
                    continue;
                }

                let extension = match entry.path().extension() {
                    Some(extension) => extension,
                    None => {
                        println!("Unsupported path : {}", entry.path().display());
                        continue;
                    }
                };

                match extension.to_str() {
                    Some("flac") => {
                        musics.push(Box::new(FlacFile::from_path(folder_path, entry.path())));
                    }
                    Some("mp3") => {
                        musics.push(Box::new(Mp3File::from_path(folder_path, entry.path())))
                    }
                    Some("m3u") | Some("jpg") => (),
                    _ => println!("Unsupported format : {}", entry.path().display()),
                };
            }
        }

        // let total = mp3_musics.len() + flac_musics.len();
        let total = musics.len();
        println!("Musics about to be upserted: {total}");
        let bar = ProgressBar::new(total as u64);

        bar.set_style(
            ProgressStyle::default_bar()
                .template("[{elapsed_precise}] {bar:50.cyan/blue} {pos:>7}/{len:7} {msg}")
                .unwrap()
                .progress_chars("##-"),
        );

        let ipv4 = public_ip().await?;
        let username = username().to_string();

        upsert_musics(&musics, &conn, &bar, &ipv4, &username, self.dry).await;

        bar.finish();
        Ok(())
    }
}
