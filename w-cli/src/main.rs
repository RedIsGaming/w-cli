use std::{thread, time::Duration, env};
use serde::{Serialize, Deserialize};
use serde_json::{Value, json, Error as SerdeJsonError};
use reqwest::{Error as ReqwestError, blocking};
//use tabular::{Row, Table};

#[derive(Debug, Clone, Serialize, Deserialize)]
struct WeatherApi {
    #[serde(rename = "NewEndpoint")]
    new_endpoint: bool,
    #[serde(rename = "DeviceID")]
    device_id: usize,
    #[serde(rename = "AppKey")]
    app_key: String,
    #[serde(rename = "ApiKey")]
    api_key: String,
}

fn weather_api(args: WeatherApi, macaddress: &str) -> String {
    let endpoint = if args.new_endpoint { "rt" } else { "api" };
    let response = format!("https://{}.ambientweather.net/v1/devices/{}?applicationKey={}&apiKey={}", endpoint, macaddress, 
        args.app_key.trim(), args.api_key);

    response
}

fn raw_devicedata(args: WeatherApi, macaddress: String) -> Result<Value, ReqwestError> {
    let id = args.device_id;
    let response: Value = blocking::get(weather_api(args, &macaddress))?
        .json()?;

    thread::sleep(Duration::from_millis(1000));

    match response["devices"][0]["info"]["name"].as_str() {
        Some(name) => println!("The name of the device is: {}.", name),
        None => println!("Couldn't find the name of the device."),
    }
    
    Ok(json!(response[id]))
}    

fn devicedata(args: WeatherApi) -> Result<Value, SerdeJsonError> {
    let device_data = raw_devicedata(args, "".to_string())
        .expect("Couldn't get the device data.")
        .to_owned();

    Ok(json!(device_data["lastData"]))
}

fn convert_macaddress() -> u64 {
    let macaddress = env::var("w-device")
        .expect("Couldn't retrieve the MAC address.")
        .to_owned();

    let replace_colon = macaddress
        .replace(":", "")
        .replace("-", "");

    let to_u64 = u64::from_str_radix(&replace_colon, 16);
    
    match to_u64 {
        Ok(to_usize) => println!("MAC address as usize: {}.", to_usize as usize),
        Err(ref err) => eprintln!("Failed to convert the MAC address as usize. {}", *err),
    }

    to_u64.unwrap()
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let credentials = WeatherApi {
        new_endpoint: false,
        device_id: convert_macaddress() as usize,
        app_key: env::var("w-app")?.to_owned(),
        api_key: env::var("w-api")?.to_owned(),
    };

    match devicedata(credentials) {
        Ok(value) => println!("{}", value["tempf"]),
        Err(err) => eprintln!("Error! Couldn't retrieve the credentials. {}", err),
    }

    Ok(())
}