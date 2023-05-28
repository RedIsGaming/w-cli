use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Weather {
    #[serde(rename = "Latitude")]
    pub lat: String,
    #[serde(rename = "Longitude")]
    pub long: String,
    #[serde(rename = "Temperature")]
    pub temp: f32,
    #[serde(rename = "FeelsLike")]
    pub feels_like: f32,
    #[serde(rename = "Humidity")]
    pub humid: u32,
    #[serde(rename = "WindDirection")]
    pub wind_dir: u32,
    #[serde(rename = "WindSpeed")]
    pub wind_speed: f32,
    #[serde(rename = "WindGust")]
    pub wind_gust: u32,
}