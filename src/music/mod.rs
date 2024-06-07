use crate::errors::CriticalErrorKind;

pub mod flac_file;
pub mod mp3_file;

static RATINGS: &[f64] = &[0.0, 0.5, 1.0, 1.5, 2.0, 2.5, 3.0, 3.5, 4.0, 4.5, 5.0];

#[async_trait::async_trait]
pub trait Music {
    fn path(&self) -> &str;
    fn folder(&self) -> &str;
    fn length(&self) -> i64;
    fn artist(&self) -> &str;
    fn album(&self) -> &str;
    fn title(&self) -> &str;
    fn genre(&self) -> &str;
    fn track(&self) -> i64;
    fn rating(&self) -> Result<f64, CriticalErrorKind>;
    fn keywords(&self) -> Vec<String>;
    fn links(&self) -> Vec<String>;
    async fn size(&self) -> Result<u64, CriticalErrorKind> {
        match async_fs::metadata(self.path()).await {
            Ok(metadata) => Ok(metadata.len()),
            Err(e) => return Err(CriticalErrorKind::IOError(e)),
        }
    }
}

impl std::fmt::Debug for dyn Music + Send + Sync {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let rt = tokio::runtime::Builder::new_current_thread()
            .enable_all()
            .build()
            .unwrap();

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

impl std::fmt::Display for dyn Music + Send + Sync {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.path())
    }
}
