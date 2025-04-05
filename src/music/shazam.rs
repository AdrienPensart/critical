use super::errors::CriticalErrorKind;
use crate::fingerprinting::communication::recognize_song_from_signature;
use crate::fingerprinting::signature_format::DecodedSignature;
use serde_json::Value;

#[derive(clap::Parser)]
#[clap(about = "Detect song")]
pub struct Shazam {
    pub file: String,
}

pub struct SongRecognizedMessage {
    pub path: String,
    pub artist_name: String,
    pub album_name: Option<String>,
    pub song_name: String,
}

pub async fn try_recognize_song(
    path: String,
    signature: &DecodedSignature,
) -> Result<SongRecognizedMessage, CriticalErrorKind> {
    let json_object = recognize_song_from_signature(signature).await?;
    let mut album_name: Option<String> = None;
    if let Value::Array(sections) = &json_object["track"]["sections"] {
        for section in sections {
            if let Value::String(string) = &section["type"] {
                if string == "SONG" {
                    if let Value::Array(metadata) = &section["metadata"] {
                        for metadatum in metadata {
                            if let Value::String(title) = &metadatum["title"] {
                                if title == "Album" {
                                    if let Value::String(text) = &metadatum["text"] {
                                        album_name = Some(text.to_string());
                                    }
                                }
                            }
                        }
                        break;
                    }
                }
            }
        }
    }
    Ok(SongRecognizedMessage {
        path: path.clone(),
        artist_name: match &json_object["track"]["subtitle"] {
            Value::String(string) => string.to_string(),
            _ => {
                return Err(CriticalErrorKind::NoMatch { path });
            }
        },
        album_name,
        song_name: match &json_object["track"]["title"] {
            Value::String(string) => string.to_string(),
            _ => {
                return Err(CriticalErrorKind::NoMatch { path });
            }
        },
    })
}
