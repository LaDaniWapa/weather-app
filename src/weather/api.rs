use crate::errors::AppError;
use crate::weather::models::{Forecast, GeocodingApiResult};
use reqwest::get;

pub async fn get_coordinates(city: &str) -> Result<GeocodingApiResult, AppError> {
    let url = format!("https://geocoding-api.open-meteo.com/v1/search?name={city}",);

    Ok(get(url).await?.json::<GeocodingApiResult>().await?)
}

pub async fn get_forecast(lat: &f32, lon: &f32) -> Result<Forecast, AppError> {
    let url = format!(
        "https://api.open-meteo.com/v1/forecast?latitude={lat}&longitude={lon}&hourly=temperature_2m,weathercode,windspeed_10m"
    );

    Ok(get(url).await?.json::<Forecast>().await?)
}
