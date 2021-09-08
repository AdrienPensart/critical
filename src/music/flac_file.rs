use std::path::{PathBuf, Path};
use crate::music::Music;
use metaflac::Tag as FlacTag;
use metaflac::block::VorbisComment;

pub struct FlacFile {
    path: PathBuf,
    tag: FlacTag
}

impl FlacFile {
    pub fn from_path(path: &Path) -> FlacFile{
        FlacFile {
            path: path.to_path_buf(),
            tag: FlacTag::read_from_path(&path).unwrap()
        }
    }
    fn comments(&self) -> &VorbisComment {
        self.tag.vorbis_comments().unwrap()
    }
}

impl Music for FlacFile {
    fn path(&self) -> &str {
        self.path.to_str().unwrap()
    }

    fn duration(&self) -> i64 {
        if let Some(stream_info) = self.tag.get_streaminfo() {
             let duration = (stream_info.total_samples as i64 / stream_info.sample_rate as i64) as i64;
             return duration
        }
        0
    }

    fn artist(&self) -> &str {
        if let Some(artists) = self.comments().artist() {
            if !artists.is_empty() {
                return artists[0].as_str();
            }
        }
        ""
    }

    fn title(&self) -> &str {
        if let Some(titles) = self.comments().title() {
            if !titles.is_empty(){
                return titles[0].as_str();
            }
        }
        ""
    }

    fn album(&self) -> &str {
        if let Some(albums) = self.comments().album() {
            if !albums.is_empty() {
                return albums[0].as_str()
            }
        }
        ""
    }

    fn genre(&self) -> &str {
        if let Some(genres) = self.comments().genre() {
            if !genres.is_empty() {
                return genres[0].as_str()
            }
        }
        ""
    }

    fn track(&self) -> i64 {
        if let Some(track) = self.comments().track() {
            if let Ok(track) = track.to_string().parse::<i64>() {
                return track
            }
        }
        0
    }

    fn rating(&self) -> f64 {
        if let Some(fmps_ratings) = self.tag.get_vorbis("fmps_rating") {
            for fmps_rating in fmps_ratings {
                if let Ok(rating) = fmps_rating.to_string().parse::<f64>() {
                    return rating;
                }
            }
        }
        0.0
    }

    fn keywords(&self) -> Vec<String> {
        if let Some(descriptions) = self.tag.get_vorbis("description") {
            if let Some(description) = descriptions.into_iter().next(){
                return description.split_whitespace().map(|k| k.trim_matches(char::from(0)).to_string()).collect();
            }
        }
        Vec::new()
    }

    fn links(&self) -> Vec<String> {
        vec![String::from(self.path())]
    }
}
