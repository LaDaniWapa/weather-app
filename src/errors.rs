use thiserror::Error;

pub type Result<T> = std::result::Result<T, AppError>;

#[derive(Error, Debug)]
pub enum AppError {
    #[error("Network error: {0}")]
    Network(#[from] reqwest::Error),
    #[error("City not found: {0}")]
    CityNotFound(String),
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
}
