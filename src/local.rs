use anyhow::Result;
use walkdir::{DirEntry, WalkDir};
use indicatif::{ProgressBar, ProgressStyle};
use itertools::Itertools;
use uuid::Uuid;
use std::collections::HashMap;

use crate::commands::local::ScanOpts;
use crate::music::Music;
use crate::music::flac_file::FlacFile;
use crate::music::mp3_file::Mp3File;

fn is_hidden(entry: &DirEntry) -> bool {
    entry.file_name()
         .to_str()
         .map(|s| s.starts_with('.'))
         .unwrap_or(false)
}

pub fn scan(opts: ScanOpts) -> Result<()> {
    let user = opts.user;
    let authenticated_user = user.authenticate()?;

    let mut musics: Vec<Box<dyn Music>> = Vec::new();
    for folder in opts.folders {
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
                    musics.push(Box::new(FlacFile::from_path(entry.path())));
                },
                Some("mp3") => {
                    musics.push(Box::new(Mp3File::from_path(entry.path())));
                },
                Some("m3u") => (),
                _ => println!("Unsupported format : {}", entry.path().display())
            };

        }
    }

    println!("Music about to be upserted: {}", musics.len());
    let bar = ProgressBar::new(musics.len() as u64);
    bar.set_style(ProgressStyle::default_bar()
        .template("[{elapsed_precise}] {bar:40.cyan/blue} {pos:>7}/{len:7} {msg}")
        .progress_chars("##-"));

    for chunk in &musics.into_iter().chunks(opts.chunks) {
        let mut operations = Vec::new();
        for music in chunk {
            let uuid = Uuid::new_v4();
            let operation_name = format!("music_{}", uuid.to_simple());
            let upsert_music_query = music.create_upsert_query3(authenticated_user.user_id, &operation_name);
            let mut operation = HashMap::new();
            operation.insert("query", upsert_music_query);
            operation.insert("operationName", operation_name);
            operations.push(operation);
        }
        let request_body = serde_json::to_string_pretty(&operations).unwrap();

        let _music_upsert_response = authenticated_user.client.post(&user.user_login.endpoint)
            .header(reqwest::header::CONTENT_TYPE, "application/json; charset=utf-8")
            .body(request_body)
            .send()?;
        bar.inc(opts.chunks as u64);
    }
    bar.finish();
    Ok(())
}
