use id3::Tag as Mp3Tag;
use id3::TagLike;
use mp3_duration;

use crate::errors::CriticalErrorKind;
use crate::music::{Music, RATINGS};

pub struct Mp3File {
    folder: String,
    path: String,
    tag: Mp3Tag,
}

impl Mp3File {
    pub fn from_path(folder: &str, path: &str) -> Result<Mp3File, CriticalErrorKind> {
        Ok(Mp3File {
            folder: folder.to_string(),
            path: path.to_string(),
            tag: Mp3Tag::read_from_path(path)?,
        })
    }
}

impl Music for Mp3File {
    fn path(&self) -> &str {
        &self.path
    }

    fn folder(&self) -> &str {
        &self.folder
    }

    fn length(&self) -> i64 {
        if let Ok(duration) = mp3_duration::from_path(&self.path) {
            duration.as_secs() as i64
        } else {
            0
        }
    }

    fn artist(&self) -> &str {
        self.tag.artist().unwrap_or_default()
    }

    fn title(&self) -> &str {
        self.tag.title().unwrap_or_default()
    }

    fn album(&self) -> &str {
        self.tag.album().unwrap_or_default()
    }

    fn genre(&self) -> &str {
        self.tag.genre().unwrap_or_default()
    }

    fn track(&self) -> i64 {
        if let Some(track) = self.tag.track() {
            if let Ok(number) = track.to_string().parse::<i64>() {
                number
            } else {
                0
            }
        } else {
            0
        }
    }

    fn rating(&self) -> Result<f64, CriticalErrorKind> {
        for frame in self.tag.frames() {
            if let Some(extended_text) = frame.content().extended_text() {
                if extended_text.description == "FMPS_Rating" {
                    if let Ok(mut rating) = extended_text.value.to_string().parse::<f64>() {
                        rating *= 5.0;
                        if !RATINGS.contains(&rating) {
                            return Err(CriticalErrorKind::InvalidRating {
                                path: self.path().to_string(),
                                rating,
                            });
                        }
                        return Ok(rating);
                    }
                }
            }
        }
        Ok(0.0)
    }

    fn keywords(&self) -> Vec<String> {
        for comment in self.tag.comments() {
            if comment.lang == "eng" {
                return comment
                    .text
                    .split_whitespace()
                    .map(|k| k.trim_matches(char::from(0)).to_string())
                    .collect();
            }
        }
        Vec::new()
    }
}
