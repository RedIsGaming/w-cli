use serde::{Serialize, Deserialize};

use reqwest::blocking::Client;
use serde_json::{self, Value, json};

use ambient_weather_api::*;
use std::env;

#[derive(Debug, Serialize, Deserialize)]
struct PlaceApi {
    #[serde(rename = "Endpoint")]
    endpoint: String,
    #[serde(rename = "Response")]
    response: String,
}

#[allow(unused)]
impl PlaceApi {
    fn new<T: Into<String>>(args: impl Iterator<Item = T>) -> Result<Self, &'static str> {
        let mut args = args.skip(1);

        let country = args.next().ok_or("Country not found")?.into();
        let city = args.next().ok_or("Place not found")?.into();

        let place_api = PlaceApi {
            endpoint: format!("https://api.example.com/{}/{}", country, city),
            response: String::new(), // Placeholder value
        };

        /*Ok(match place_api {
            let connect = reqwest::blocking::get(format!()).unwrap();
        })*/

        let client = Client::new();
        let response = client.get("https://jsonplaceholder.typicode.com/todos?userId=1").send().unwrap();

        /*match response {
            print!("{:?}", response.text().unwrap());
        }*/

        Ok((place_api))
    } //
} //

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