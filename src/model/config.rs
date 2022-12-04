use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Config {
    pub postgres: Postgres,
    pub redis: Redis,
    pub map: Map,
    pub oss: Oss,
}

#[derive(Serialize, Deserialize)]
pub struct Postgres {
    pub driver: String,
    pub url: String,
    pub db: String,
    pub port: i64,
    pub username: String,
    pub password: String,
}

#[derive(Serialize, Deserialize)]
pub struct Redis {
    pub url: String,
    pub port: i64,
    pub password: String,
}

#[derive(Serialize, Deserialize)]
pub struct Map {
    pub secret: String,
}

#[derive(Serialize, Deserialize)]
pub struct Oss {
    pub access_key: String,
    pub secret_key: String,
    pub bucket_name: String
}