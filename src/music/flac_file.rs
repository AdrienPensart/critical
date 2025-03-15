use metaflac::block::VorbisComment;
use metaflac::Tag as FlacTag;

use super::errors::CriticalErrorKind;
use super::ratings::Rating;
use super::Music;

pub struct FlacFile {
    folder: String,
    path: String,
    tag: FlacTag,
    comments: VorbisComment,
}

impl FlacFile {
    pub fn from_path(folder: &str, path: &str) -> Result<FlacFile, CriticalErrorKind> {
        let tag = FlacTag::read_from_path(path)?;
        let Some(comments) = tag.vorbis_comments() else {
            return Err(CriticalErrorKind::FlacCommentsError);
        };
        Ok(FlacFile {
            folder: folder.to_string(),
            path: path.to_string(),
            comments: comments.clone(),
            tag,
        })
    }
}

impl Music for FlacFile {
    fn path(&self) -> &str {
        &self.path
    }

    fn folder(&self) -> &str {
        &self.folder
    }

    fn length(&self) -> i64 {
        if let Some(stream_info) = self.tag.get_streaminfo() {
            stream_info.total_samples as i64 / stream_info.sample_rate as i64
        } else {
            0
        }
    }

    fn artist(&self) -> &str {
        if let Some(artists) = self.comments.artist() {
            if !artists.is_empty() {
                return artists[0].as_str();
            }
        }
        ""
    }

    fn title(&self) -> &str {
        if let Some(titles) = self.comments.title() {
            if !titles.is_empty() {
                return titles[0].as_str();
            }
        }
        ""
    }

    fn album(&self) -> &str {
        if let Some(albums) = self.comments.album() {
            if !albums.is_empty() {
                return albums[0].as_str();
            }
        }
        ""
    }

    fn genre(&self) -> &str {
        if let Some(genres) = self.comments.genre() {
            if !genres.is_empty() {
                return genres[0].as_str();
            }
        }
        ""
    }

    fn track(&self) -> i64 {
        if let Some(track) = self.comments.track() {
            if let Ok(track) = track.to_string().parse::<i64>() {
                return track;
            }
        }
        0
    }

    fn rating(&self) -> Result<Rating, CriticalErrorKind> {
        if let Some(fmps_ratings) = self.tag.get_vorbis("fmps_rating") {
            for fmps_rating in fmps_ratings {
                if let Ok(mut rating) = fmps_rating.to_string().parse::<f64>() {
                    rating *= 5.0;
                    let rating = Rating::try_from(rating)?;
                    return Ok(rating);
                }
            }
        }
        Ok(Rating::default())
    }

    fn keywords(&self) -> Vec<String> {
        if let Some(descriptions) = self.tag.get_vorbis("description") {
            if let Some(description) = descriptions.into_iter().next() {
                return description
                    .split_whitespace()
                    .map(|k| k.trim_matches(char::from(0)).to_string())
                    .collect();
            }
        }
        Vec::new()
    }
}
