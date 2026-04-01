use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct GeocodingApiResult {
    pub results: Option<Vec<GeocodingResults>>,
}

#[derive(Deserialize)]
pub struct GeocodingResults {
    pub name: String,
    pub latitude: f32,
    pub longitude: f32,
}

#[derive(Deserialize, Serialize)]
pub struct Forecast {
    pub hourly_units: HourlyUnits,
    pub hourly: HourlyData,
}

#[derive(Deserialize, Serialize)]
pub struct HourlyUnits {
    pub time: String,
    #[serde(rename = "temperature_2m")]
    pub temperature: String,
    #[serde(rename = "weathercode")]
    pub weather_code: String,
    #[serde(rename = "windspeed_10m")]
    pub wind_speed: String,
}

#[derive(Deserialize, Serialize)]
pub struct HourlyData {
    pub time: Vec<String>,
    #[serde(rename = "temperature_2m")]
    pub temperature: Vec<f32>,
    #[serde(rename = "weathercode")]
    pub weather_code: Vec<u8>,
    #[serde(rename = "windspeed_10m")]
    pub wind_speed: Vec<f32>
}
