use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Address {
    pub status: String,
    pub city: String,
    pub rectangle: String,
    pub info: String,
    pub infocode: String,
    pub adcode: String,
    pub province: String
}