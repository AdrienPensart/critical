use super::{errors::CriticalErrorKind, ratings::Rating};

pub type BoxMusicFile = Box<dyn MusicFile + Send + Sync>;

#[async_trait::async_trait]
pub trait MusicFile {
    fn path(&self) -> &str;
    fn folder(&self) -> &str;
    fn length(&self) -> i64;
    fn artist(&self) -> &str;
    fn album(&self) -> &str;
    fn title(&self) -> &str;
    fn genre(&self) -> &str;
    fn track(&self) -> i64;
    fn rating(&self) -> Result<Rating, CriticalErrorKind>;
    fn keywords(&self) -> Vec<String>;
    async fn size(&self) -> Result<u64, CriticalErrorKind> {
        Ok(async_fs::metadata(self.path()).await?.len())
    }
    fn links(&self) -> Vec<String> {
        vec![String::from(self.path())]
    }
}

impl std::fmt::Debug for dyn MusicFile + Sync {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let rt = tokio::runtime::Builder::new_current_thread()
            .enable_all()
            .build()
            .map_err(|_| std::fmt::Error)?;

        let size = rt.block_on(self.size());

        f.debug_struct("Music")
            .field("path", &self.path())
            .field("folder", &self.folder())
            .field("size", &size)
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

impl std::fmt::Display for dyn MusicFile {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.path())
    }
}
