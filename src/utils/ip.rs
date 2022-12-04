use std::collections::HashMap;
use axum::{Extension, response};
use log::{info};
use reqwest_middleware::ClientWithMiddleware;
use serde_json::{json, Value};
use crate::model::address::Address;

pub async fn parse_ip(Extension(ref client): Extension<ClientWithMiddleware>) -> response::Json<Value> {
    let res = client.get("https://restapi.amap.com/v3/ip?ip=114.247.50.2&key=3c13cd7bb5b51d6d5633328ee3ff31be")
        .send()
        .await.unwrap();
    let x = res.json::<Address>().await.unwrap();
    response::Json(json!(x))
}