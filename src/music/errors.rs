#[derive(thiserror::Error, Debug)]
pub enum CriticalErrorKind {
    #[error("Request error")]
    ReqwestError(#[from] reqwest::Error),
    #[error("Invalid request header")]
    HeaderError(#[from] reqwest::header::ToStrError),
    #[error("Invalid music rating for {path} : {rating}")]
    InvalidRating { path: String, rating: f64 },
    #[error("Invalid min/max rating, minimum rating {min_rating} should be < {max_rating}")]
    InvalidMinMaxRating { min_rating: f64, max_rating: f64 },
    #[error("Invalid min/max length, minimum length {min_length} should be < {max_length}")]
    InvalidMinMaxLength { min_length: i64, max_length: i64 },
    #[error("Invalid min/max size, minimum size {min_size} should be < {max_size}")]
    InvalidMinMaxSize { min_size: i64, max_size: i64 },
    #[error("Public IP not detected")]
    NoPublicIp,
    #[error("EdgeDB error")]
    EdgedbError(#[from] edgedb_tokio::Error),
    #[error("IO error: {0}")]
    IOError(#[from] std::io::Error),
    #[error("Upsert operation failed for path: {path} with object: {object}")]
    UpsertError { path: String, object: String },
    #[error("Invalid ID3 tag")]
    Mp3TagError(#[from] id3::Error),
    #[error("Invalid Flac tag")]
    FlacTagError(#[from] metaflac::Error),
    #[error("Invalid Flac comments")]
    FlacCommentsError,
    #[error("Invalid progress bar template")]
    ProgressBarError(#[from] indicatif::style::TemplateError),
    #[error("Semaphore error")]
    SemaphoreError(#[from] tokio::sync::AcquireError),
    #[error("JSON serialization error")]
    SerializationError(#[from] serde_json::Error),
    #[error("Relative path error")]
    RelativePathError(#[from] std::path::StripPrefixError),
}
