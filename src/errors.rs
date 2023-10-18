use thiserror::Error;
use reqwest::Error as RequestError;
use reqwest::header::ToStrError;
use edgedb_tokio::Error as EdgedbError;
use std::path::PathBuf;

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
}
