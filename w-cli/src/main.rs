use ambient_weather_api::*;
use std::env;

use serde::{
    Serialize, 
    Deserialize,
};

#[derive(Serialize, Deserialize, Debug)]
#[allow(unused)]
struct Place {
    country: String,
    place: String,
}

#[allow(unused)]
impl Place {
    fn new(args: impl Iterator<Item = String>) -> Result<Place, &'static str> {
        let mut args = args.skip(1);

        let country = args.next().ok_or("Country not found")?;
        let place = args.next().ok_or("Place not found")?;

        Ok(Place {
            country,
            place,
        })
    }
}

fn main() {
    let api_credentials = AmbientWeatherAPICredentials {
        api_key: env::var("w-api").expect("API-Key couldn't be found"),
        app_key: env::var("w-app").expect("App-Key couldn't be found"),
        device_id: 0,
        use_new_api_endpoint: false,
    };

    let latest_data = get_latest_aw_device_data(api_credentials);
    println!("{}", latest_data["tempf"]);
} //