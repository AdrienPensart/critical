use base64::DecodeError;

#[derive(thiserror::Error, Debug)]
pub enum CriticalErrorKind {
    #[error("Formatting error")]
    FormatError(#[from] std::fmt::Error),
    #[error("Request error")]
    ReqwestError(#[from] reqwest::Error),
    #[error("Invalid request header")]
    HeaderError(#[from] reqwest::header::ToStrError),
    #[error("Invalid request header")]
    InvalidHeader(#[from] reqwest::header::InvalidHeaderValue),
    #[error("Invalid music rating for {path} : {rating}")]
    InvalidRating { path: String, rating: f64 },
    #[error("Invalid min/max rating, minimum rating {min_rating} should be < {max_rating}")]
    InvalidMinMaxRating { min_rating: f64, max_rating: f64 },
    #[error("Invalid min/max length, minimum length {min_length} should be < {max_length}")]
    InvalidMinMaxLength { min_length: i64, max_length: i64 },
    #[error("Invalid min/max size, minimum size {min_size} should be < {max_size}")]
    InvalidMinMaxSize { min_size: i64, max_size: i64 },
    #[error("Interleave error")]
    InterleaveError,
    #[error("Public IP not detected")]
    NoPublicIp,
    #[error("EdgeDB error: {0}")]
    GelError(#[from] gel_tokio::Error),
    #[error("Gel DB error with {object}: {error}")]
    GelErrorWithObject {
        error: gel_tokio::Error,
        object: String,
    },
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
    #[error("IndraDB datastore error")]
    DatastoreError(#[from] rmp_serde::decode::Error),
    #[error("Music file size too large")]
    FileSizeError(#[from] std::num::TryFromIntError),
    #[error("Music song not matched")]
    NoMatch { path: String },
    #[error("Time error")]
    TimeError(#[from] std::time::SystemTimeError),
    #[error("Base64 encode/decore error")]
    Base64Error(#[from] DecodeError),
    #[error("Rodio decoder error")]
    DecoderError(#[from] rodio::decoder::DecoderError),
    #[error("Invalid sample rate")]
    InvalidSampleRate(u32),
    #[error("Invalid frequency band")]
    InvalidFrequencyBand(u32),
    #[error("Invalid pass number")]
    InvalidPassNumber(u32),
    #[error("Invalid data length")]
    InvalidDataLength(usize),
    #[error("Invalid magic number")]
    InvalidMagicNumber(u32),
    #[error("Invalid header size")]
    InvalidHeaderSize(u32),
    #[error("Invalid CRC32")]
    InvalidCRC32(u32),
    #[error("Invalid URI")]
    InvalidURI(String),
}
