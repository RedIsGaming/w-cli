pub mod place;
pub mod weather;
pub mod temp;

use crate::{place::Place, weather::Weather, temp::TempDisplay};

#[derive(Debug)]
pub struct Location {
    pub country: String,
    pub city: String,
}

fn location(places: &Place) -> (&String, &String) {
    (&places.country, &places.city)
}

impl From<TempDisplay> for Location {
    fn from(args: TempDisplay) -> Self {
        let places_location = &args.temp
            .iter()
            .next()
            .and_then(|(_, t)| Some(&t.places))
            .unwrap();

        let (country, city) = location(places_location);

        Self { 
            country: country.clone(),
            city: city.clone(), 
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

fn temperature(weather: &Weather) -> (&String, &String, f32, f32, u32, u32, f32, u32) {
    (
        &weather.lat, &weather.long, 
        weather.temp, weather.feels_like, 
        weather.humid, weather.wind_dir, 
        weather.wind_speed, weather.wind_gust
    )
}

impl From<TempDisplay> for Temperature {
    fn from(args: TempDisplay) -> Self {
        let weather_temperature = &args.temp
            .iter()
            .next()
            .and_then(|(_, t)| Some(&t.weather))
            .unwrap();

        //Fix this too
        let (
                lat, long, 
                temp, feels_like, 
                humid, wind_dir, 
                wind_speed, wind_gust
            ) = temperature(&weather_temperature);

        //Also fix this
        Self { 
            lat: lat.clone(),
            long: long.clone(),
            temp,
            feels_like,
            humid,
            wind_dir,
            wind_speed,
            wind_gust,
        }
    } //
} //