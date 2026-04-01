use axum::response::{IntoResponse, Response};
use reqwest::StatusCode;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum AppError {
    #[error("Network error: {0}")]
    Network(#[from] reqwest::Error),
    #[error("City not found: {0}")]
    CityNotFound(String),
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        match self {
            AppError::Network(err) => (StatusCode::BAD_GATEWAY, err.to_string()),
            AppError::CityNotFound(city) => {
                (StatusCode::NOT_FOUND, format!("City not found \"{city}\""))
            }
        }
        .into_response()
    }
}
