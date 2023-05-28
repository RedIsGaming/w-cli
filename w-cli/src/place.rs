use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Place {
    #[serde(rename = "Country")]
    pub country: String,
    #[serde(rename = "City")]
    pub city: String,
}