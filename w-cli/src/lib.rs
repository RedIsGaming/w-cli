pub mod place;
pub mod weather;

use crate::{place::Place, weather::Weather};
use serde::{Serialize, Deserialize};
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize)]
pub struct Temp {
    #[serde(rename = "Places")]
    pub places: Place,
    #[serde(rename = "Weather")]
    pub weather: Weather,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TempDisplay {
    #[serde(rename = "Temp")]
    pub temp: HashMap<String, Temp>,
}

#[derive(Debug)]
pub struct Location {
    pub country: String,
    pub city: String,
}

fn location(places: &Place) -> (String, String) {
    (places.country.clone(), places.city.clone())
}

impl From<TempDisplay> for Location {
    fn from(args: TempDisplay) -> Self {
        let places_location = &args.temp
            .iter()
            .next()
            .and_then(|(_, t)| Some(&t.places))
            .unwrap();

        let (country, city) = location(&places_location);

        Self { 
            country,
            city, 
        }
    }
}

//Fix this struct
#[derive(Debug)]
pub struct Temperature {
    pub lat: String,
    pub long: String,
    pub temp: f32,
    pub feels_like: f32,
    pub humid: u32,
    pub wind_dir: u32,
    pub wind_speed: f32,
    pub wind_gust: u32,
}

//Fix this too
fn temperature(weather: &Weather) -> (String, String, f32, f32, u32, u32, f32, u32) {
    (weather.lat.clone(), weather.long.clone(), weather.temp.clone(), weather.feels_like.clone(), 
    weather.humid.clone(), weather.wind_dir.clone(), weather.wind_speed.clone(), weather.wind_gust.clone())
}

//Also fix this
impl From<TempDisplay> for Temperature {
    fn from(args: TempDisplay) -> Self {
        let weather_temperature = &args.temp
            .iter()
            .next()
            .and_then(|(_, t)| Some(&t.weather))
            .unwrap();

        let (lat, long, temp, feels_like, 
            humid, wind_dir, wind_speed, wind_gust) = temperature(&weather_temperature);

        Self { 
            lat,
            long,
            temp,
            feels_like,
            humid,
            wind_dir,
            wind_speed,
            wind_gust,
        }
    }
}