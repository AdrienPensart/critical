use id3::Tag as Mp3Tag;
use id3::TagLike;
use mp3_duration;
use num_traits::ToPrimitive;

use super::errors::CriticalErrorKind;
use super::ratings::Rating;
use super::Music;

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
            duration.as_secs().to_i64().unwrap_or_default()
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
            track.to_string().parse::<i64>().unwrap_or_default()
        } else {
            0
        }
    }

    fn rating(&self) -> Result<Rating, CriticalErrorKind> {
        for frame in self.tag.frames() {
            if let Some(extended_text) = frame.content().extended_text() {
                if extended_text.description == "FMPS_Rating" {
                    if let Ok(mut rating) = extended_text.value.to_string().parse::<f64>() {
                        rating *= 5.0;
                        let rating = Rating::try_from(rating)?;
                        return Ok(rating);
                    }
                }
            }
        }
        Ok(Rating::default())
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
