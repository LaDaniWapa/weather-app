mod weather;
mod errors;

use axum::{Router, routing::get, extract::Query, Json};
use tower_http::{cors::CorsLayer};
use serde::Deserialize;
use crate::errors::AppError;
use crate::weather::api::{get_forecast, get_coordinates};
use crate::weather::models::{Forecast, GeocodingApiResult, GeocodingResults};

fn create_route() -> Router {
    Router::new()
        .route("/", get(|| async { "Hello, World!" }))
        .route("/weather", get(get_weather))
        .layer(CorsLayer::permissive())
}

#[tokio::main]
async fn main() {
    let app = create_route();

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    println!("Server running on http://localhost:3000");
    axum::serve(listener, app).await.unwrap();
}

#[derive(Deserialize)]
pub struct WeatherParams {
    pub city: String,
}

async fn get_weather(Query(params): Query<WeatherParams>) -> Result<Json<Forecast>, AppError> {
    let api_results: GeocodingApiResult = get_coordinates(&params.city).await?;
    let coords: &GeocodingResults = &api_results.results.ok_or(AppError::CityNotFound(params.city))?[0];

    let forecast: Forecast = get_forecast(&coords.latitude, &coords.longitude).await?;

    Ok(Json(forecast))
}