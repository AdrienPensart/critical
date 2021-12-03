use anyhow::{Result, Context, bail};
use clap::Parser;
use walkdir::{DirEntry, WalkDir};
use indicatif::{ProgressBar, ProgressStyle};
use itertools::Itertools;
use uuid::Uuid;
use std::collections::HashMap;
use graphql_client::Response;

use crate::err_on_some::ErrOnSome;
use crate::user::User;
use crate::music::{Music, upsert_music};
use crate::music::flac_file::FlacFile;
use crate::music::mp3_file::Mp3File;

#[derive(Parser, Debug)]
#[clap(about = "Scan folders and save music")]
pub struct FoldersScanner {
    /// Enable bulk insert / batch
    #[clap(short, long, visible_alias = "batch")]
    pub bulk: bool,

    /// Upsert chunks
    #[clap(long, default_value = "200", long)]
    pub chunks: usize,

    /// MusicBot user
    #[clap(flatten)]
    pub user: User,

    /// Clean musics before scanning
    #[clap(short, long)]
    pub clean: bool,

    pub folders: Vec<String>
}

fn is_hidden(entry: &DirEntry) -> bool {
    entry.file_name()
         .to_str()
         .map(|s| s.starts_with('.'))
         .unwrap_or(false)
}

impl FoldersScanner {
    pub fn scan(self) -> Result<()> {
        let authenticated_user = self.user.authenticate()?;

        let mut musics: Vec<Box<dyn Music>> = Vec::new();
        for folder in self.folders {
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

        if self.bulk {
            for chunk in &musics.into_iter().chunks(self.chunks) {
                let mut operations = Vec::new();
                for music in chunk {
                    let uuid = Uuid::new_v4();
                    let operation_name = format!("music_{}", uuid.to_simple());
                    let upsert_music_query = music.create_bulk_upsert_query(authenticated_user.user_id, &operation_name);
                    let mut operation = HashMap::new();
                    operation.insert("query", upsert_music_query);
                    operation.insert("operationName", operation_name);
                    operations.push(operation);
                }
                let request_body = serde_json::to_string_pretty(&operations)?;

                let _music_upsert_response = authenticated_user
                    .post()
                    .header(reqwest::header::CONTENT_TYPE, "application/json; charset=utf-8")
                    .body(request_body)
                    .send()?
                    .error_for_status()?;

                bar.inc(self.chunks as u64);
            }
        } else {
            for music in musics {
                let request_body = music.create_upsert_query(authenticated_user.user_id);
                let response_body: Response<upsert_music::ResponseData> = authenticated_user
                    .post()
                    .json(&request_body)
                    .send()?
                    .error_for_status()?
                    .json()?;

                response_body.errors.err_on_some(|| bail!("{:?}", response_body.errors))?;
                let response_copy = format!("{:?}", response_body.data);

                let _client_mutation_id = response_body
                    .data.with_context(|| format!("missing music response data : {}", response_copy))?
                    .upsert_music.with_context(|| format!("missing upsert music response : {}", response_copy))?
                    .client_mutation_id;
                bar.inc(1);
            }
        }
        bar.finish();
        Ok(())
    }
}
