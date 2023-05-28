use serde::{Serialize, Deserialize};
use reqwest::blocking::Client;
use serde_json::{self, Value, json};
use std::{thread, time::Duration, env};

#[derive(Debug, Serialize, Deserialize)]
struct PlaceApi {
    #[serde(rename = "Endpoint")]
    endpoint: String,
    #[serde(rename = "Response")]
    response: String,
}

#[derive(Debug, Clone)]
struct WeatherApi {
    new_endpoint: bool,
    device_id: usize,
    app_key: String,
    api_key: String,
}

fn weather_api(args: WeatherApi, macaddress: &str) -> String {
    let endpoint = args.new_endpoint
        .map(|new_endpoint| if new_endpoint { "rt" } else { "api" })
        .unwrap_or("api");

    let response = format!("https://{}.ambientweather.net/v1/devices/{}?applicationKey={}&apiKey={}", endpoint, macaddress, 
    args.app_key, args.api_key);

    return response;
}

fn raw_devicedata(args: WeatherApi, macaddress: String) -> Result<Value, reqwest::Error> {
    let id = args.device_id;

    let response: Value = reqwest::blocking::get(weather_api(args, &macaddress))?
        .json()?;

    thread::sleep(Duration::from_millis(1000));
    return Ok(json!(response[id]))
}

fn devicedata(args: WeatherApi) -> Value {
    let device_data = raw_devicedata(args, "".to_string()).unwrap();
    return json!(device_data["lastData"])
}

fn weather_api_credentials() {
    let credentials = WeatherApi {
        api_key: env::var("w-api").expect("API-Key couldn't be found"),
        app_key: env::var("w-app").expect("App-Key couldn't be found"),
        device_id: 0,
        new_endpoint: false,
    };

    let latest_data = devicedata(credentials);
    //println!("{:?}", g);
    //println!("{}", g["tempf"]);
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

        Ok((place_api))
    } //
} //

//https://jsonplaceholder.typicode.com/todos?userId=1

fn location_api() -> reqwest::blocking::Response {
    let client = Client::new();
    let response = client.get("https://rt.ambientweather.net/v1/devices{?apiKey,applicationKey}").send().unwrap();

    match response.status() {
        reqwest::StatusCode::OK => println!("Successfully connected to the API with statuscode: {}", response.status()),
        _ => println!("Error! Couldn't retrieve a successfull connection. The statuscode was: {}", response.status()),
    }

    response
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    location_api();
    weather_api_credentials();

    let data = r#"
    {
        "name": "John Doe",
        "age": 43,
        "phones": [
            "+44 1234567",
            "+44 2345678"
        ]
    }"#;

    let v: Value = serde_json::from_str(data)?;
    println!("Please call {} at the number {}", v["name"], v["phones"][0]);

    let j = serde_json::to_string(&v)?;
    println!("{}", j);

    Ok(())
} //