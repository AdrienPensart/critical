use crate::music::errors::CriticalErrorKind;
use crate::music::playlist::Kind;
use crate::music::Music;
use edgedb_derive::Queryable;
use serde::Serialize;
use std::hash::{Hash, Hasher};
use tabled::Tabled;

#[derive(Queryable, Serialize, Clone)]
pub struct FolderResult {
    pub name: String,
    pub username: String,
    pub ipv4: String,
    pub path: String,
}

impl FolderResult {
    pub fn effective_path(&self, relative: bool) -> Result<String, CriticalErrorKind> {
        let path = if relative {
            let base = std::path::Path::new(&self.name);
            let path = std::path::Path::new(&self.path).strip_prefix(base)?;
            path.display().to_string()
        } else {
            self.path.clone()
        };
        Ok(path.replace(' ', "\\ "))
    }

    pub fn http_link(&self, relative: bool) -> Result<String, CriticalErrorKind> {
        Ok(format!(
            "http://{}/{}",
            self.ipv4,
            self.effective_path(relative)?
        ))
    }

    pub fn local_ssh_link(&self) -> String {
        format!("{}@localhost:{}", self.username, self.path)
    }

    pub fn remote_ssh_link(&self) -> String {
        format!("{}@{}:{}", self.username, self.ipv4, self.path)
    }

    pub fn links(&self, relative: bool, kinds: &[Kind]) -> Result<Vec<String>, CriticalErrorKind> {
        let mut paths = Vec::new();
        if kinds.contains(&Kind::All) || kinds.contains(&Kind::LocalSSH) {
            paths.push(self.local_ssh_link())
        }
        if kinds.contains(&Kind::All) || kinds.contains(&Kind::RemoteSSH) {
            paths.push(self.remote_ssh_link())
        }
        if kinds.contains(&Kind::All) || kinds.contains(&Kind::LocalHTTP) {
            paths.push(self.http_link(relative)?)
        }
        if kinds.contains(&Kind::All) || kinds.contains(&Kind::RemoteHttp) {
            paths.push(self.http_link(relative)?)
        }
        if kinds.contains(&Kind::All) || kinds.contains(&Kind::Local) {
            paths.push(self.effective_path(relative)?)
        }
        if kinds.contains(&Kind::All) || kinds.contains(&Kind::Remote) {
            paths.push(self.effective_path(relative)?)
        }
        Ok(paths)
    }
}

#[derive(Queryable, Serialize, Tabled, Clone)]
pub struct MusicResult {
    #[tabled(display_with("Self::display_name_and_paths", self))]
    pub name: String,

    #[tabled(display_with("Self::display_artist_album_genre", self))]
    pub artist_name: String,
    #[tabled(skip)]
    pub album_name: String,
    #[tabled(skip)]
    pub genre_name: String,

    #[tabled(skip)]
    pub length: i64,
    #[tabled(skip)]
    pub human_duration: String,

    #[tabled(skip)]
    pub size: i64,
    #[tabled(display_with("Self::display_size_duration_track_rating_keywords", self))]
    pub human_size: String,
    #[tabled(skip)]
    pub track: i64,
    #[tabled(skip)]
    pub rating: f64,

    #[tabled(skip)]
    pub keywords_names: Vec<String>,
    #[tabled(skip)]
    pub folders: Vec<FolderResult>,
}

impl PartialEq for MusicResult {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
            && self.artist_name == other.artist_name
            && self.album_name == other.album_name
            && self.genre_name == other.genre_name
    }
}
impl Eq for MusicResult {}
impl Hash for MusicResult {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.name.hash(state);
        self.artist_name.hash(state);
        self.album_name.hash(state);
        self.genre_name.hash(state);
    }
}

impl MusicResult {
    fn display_name_and_paths(&self) -> String {
        let paths = self
            .folders
            .iter()
            .map(|f| f.path.clone())
            .collect::<Vec<_>>()
            .join("\n");
        format!("{}\n{paths}", self.name)
    }

    fn display_artist_album_genre(&self) -> String {
        format!(
            "Artist: {}\nAlbum: {}\nGenre: {}",
            self.artist_name, self.album_name, self.genre_name
        )
    }

    fn display_size_duration_track_rating_keywords(&self) -> String {
        format!(
            "Size:{}\nLength: {}\nTrack: {}\nRating: {}\nKeywords: {}",
            self.human_size,
            self.human_duration,
            self.track,
            self.rating,
            self.keywords_names.join("\n"),
        )
    }

    pub fn all_links(
        &self,
        relative: bool,
        kinds: &[Kind],
    ) -> Result<Vec<String>, CriticalErrorKind> {
        let mut links = Vec::new();
        for folder in &self.folders {
            links.extend_from_slice(folder.links(relative, kinds)?.as_slice());
        }
        Ok(links)
    }
}

impl Music for MusicResult {
    fn path(&self) -> &str {
        &self.folders[0].path
    }

    fn folder(&self) -> &str {
        &self.folders[0].name
    }

    fn length(&self) -> i64 {
        self.length
    }

    fn artist(&self) -> &str {
        &self.artist_name
    }

    fn title(&self) -> &str {
        &self.name
    }

    fn album(&self) -> &str {
        &self.album_name
    }

    fn genre(&self) -> &str {
        &self.genre_name
    }

    fn track(&self) -> i64 {
        self.track
    }

    fn rating(&self) -> Result<f64, CriticalErrorKind> {
        Ok(self.rating)
    }

    fn keywords(&self) -> Vec<String> {
        self.keywords_names.clone()
    }
}

impl std::fmt::Debug for MusicResult {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Music")
            .field("path", &self.path())
            .field("folder", &self.folder())
            .field("size", &self.human_size)
            .field("artist", &self.artist())
            .field("album", &self.album())
            .field("title", &self.title())
            .field("track", &self.track())
            .field("length", &self.length())
            .field("rating", &self.rating())
            .field("keywords", &self.keywords())
            .field("links", &self.links())
            .finish()
    }
}

impl std::fmt::Display for MusicResult {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.path())
    }
}
