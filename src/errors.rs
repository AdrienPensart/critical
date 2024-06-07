use edgedb_tokio::Error as EdgedbError;
use reqwest::header::ToStrError;
use reqwest::Error as RequestError;
use std::path::PathBuf;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum CriticalErrorKind {
    #[error("Request error")]
    ReqwestError(#[from] RequestError),
    #[error("Invalid request header")]
    HeaderError(#[from] ToStrError),
    #[error("Invalid music rating for {0} : {1}")]
    InvalidRating(PathBuf, f64),
    #[error("Public IP not detected")]
    NoPublicIp,
    #[error("EdgeDB error")]
    EdgedbError(#[from] EdgedbError),
    #[error("IO error: {0}")]
    IOError(#[from] std::io::Error),
    #[error("Upsert operation failed for path: {path} with object: {object}")]
    UpsertError { path: String, object: String },
}
