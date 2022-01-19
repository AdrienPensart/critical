use std::path::{PathBuf, Path};
use crate::music::Music;
use mp3_duration;
use id3::TagLike;
use id3::Tag as Mp3Tag;

pub struct Mp3File {
    path: PathBuf,
    tag: Mp3Tag
}

impl Mp3File {
    pub fn from_path(path: &Path) -> Mp3File {
        Mp3File {
            path: path.to_path_buf(),
            tag: Mp3Tag::read_from_path(&path).unwrap()
        }
    }
}

impl Music for Mp3File {
    fn path(&self) -> &str {
        self.path.to_str().unwrap()
    }

    fn duration(&self) -> i64 {
        if let Ok(duration) = mp3_duration::from_path(&self.path) {
            duration.as_secs() as i64
        } else {
            0
        }
    }

    fn artist(&self) -> &str {
        if let Some(artist) = self.tag.artist() {
            artist
        } else {
            ""
        }
    }

    fn title(&self) -> &str {
        if let Some(title) = self.tag.title() {
            title
        } else {
            ""
        }
    }

    fn album(&self) -> &str {
        if let Some(album) = self.tag.album() {
            album
        } else {
            ""
        }
    }

    fn genre(&self) -> &str {
        if let Some(genre) = self.tag.genre() {
            genre
        } else {
            ""
        }
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

    fn rating(&self) -> f64 {
        for frame in self.tag.frames() {
            if let Some(extended_text) = frame.content().extended_text() {
                if extended_text.description == "FMPS_Rating" {
                    if let Ok(rating) = extended_text.value.to_string().parse::<f64>() {
                        return rating;
                    }
                }
            }
        }
        0.0
    }

    fn keywords(&self) -> Vec<String> {
        for comment in self.tag.comments() {
            if comment.lang == "eng" {
                return comment.text.split_whitespace().map(|k| k.trim_matches(char::from(0)).to_string()).collect();
            }
        }
        Vec::new()
    }

    fn links(&self) -> Vec<String> {
        vec![String::from(self.path())]
    }
}
