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