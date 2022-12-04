#![feature(once_cell)]
#![feature(generic_associated_types)]

extern crate core;

mod comm;
mod db;
mod entity;
mod model;
mod repository;
mod service;
mod utils;

use axum::extract::extractor_middleware;
use axum::http::StatusCode;
use axum::middleware::{AddExtension, from_extractor};
use axum::response::IntoResponse;
use axum::routing::{get, post};
use axum::{Extension, response, Router};
use log::info;
use once_cell::sync::Lazy;
use redis::{Client, Commands};
use serde_json::{json, Value};
use snowflake::{SnowflakeIdBucket, SnowflakeIdGenerator};
use std::fs::File;
use std::io::Read;
use std::net::SocketAddr;
use std::sync::{Arc, Mutex};
use reqwest_middleware::{ClientBuilder, ClientWithMiddleware};
use reqwest_retry::policies::ExponentialBackoff;
use reqwest_retry::RetryTransientMiddleware;
use reqwest_tracing::TracingMiddleware;

use crate::comm::RedisState;
use crate::db::db_conn;
//use crate::service::search_service::search_key;
use model::config::Config;
use crate::model::jwt::{authorize, Claims, JwtKey, protected};
use crate::service::storage_service::upload;
use crate::service::tag_service::create_tag;
use crate::utils::shutdown::shutdown_signal;
use sea_orm;
use sea_orm::DbConn;
use tower::{limit::ConcurrencyLimitLayer, ServiceBuilder};
use tracing::instrument::WithSubscriber;
use tracing_subscriber::util::SubscriberInitExt;
use crate::utils::ip::parse_ip;
use crate::utils::upload;

static KEY: Lazy<JwtKey> = Lazy::new(|| {
    let secret = std::env::var("JWT_SECRET").expect("JWT_SECRET must be set");
    JwtKey::new(secret.as_bytes())
});

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG)
        .with_test_writer()
        .init();
    let mut file = match File::open("conf.toml") {
        Ok(f) => f,
        Err(e) => panic!("Parse config exception:{}", e),
    };
    let mut config_str = String::new();
    match file.read_to_string(&mut config_str) {
        Ok(str) => str,
        Err(e) => panic!("Read config exception:{}", e),
    };
    let config: Config = toml::from_str(&config_str).unwrap();
    upload::upload(&config);
    upload::upload(&config);
    let secret: String = config.map.secret;
    let pool = db_conn(&config.postgres).await;
    let redis_url = format!(
        "redis://:{}@{}:{}",
        &config.redis.password, &config.redis.url, &config.redis.port
    );
    let redis = Arc::new(Mutex::new(
        Client::open(redis_url).expect("invalid connection URL"),
    ));
    let app = init_app(pool, redis);
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    tracing::debug!("liberty server start on:{}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .with_graceful_shutdown(shutdown_signal())
        .await
        .unwrap();
}

fn init_app(pool: DbConn, redis: Arc<Mutex<Client>>) -> Router {
    let api = Router::new()
        .nest("/auth", auth_api())
        .nest("/tag", tag_api());
    let retry_policy = ExponentialBackoff::builder().build_with_max_retries(3);
    let client = ClientBuilder::new(reqwest::Client::new())
        .with(TracingMiddleware::default())
        .with(RetryTransientMiddleware::new_with_policy(retry_policy))
        .build();

    let app = Router::new().nest("/api", api).layer(
        ServiceBuilder::new()
            .layer(ConcurrencyLimitLayer::new(10))
            .layer(Extension(pool))
            .layer(from_extractor::<Claims>())
            .layer(Extension(client))
            .layer((Extension(redis))),
    );
    app
}

// fn search_api() -> Router {
//     Router::new().route("/", post(search_key))
// }

fn tag_api() -> Router {
    Router::new()
        .route("/tag", post(create_tag))
        .route("/index", get(index))
        .route("/ip", get(parse_ip))
    // .route("/upload", post(upload))
    // .route("/test", post(index))
}

fn auth_api() -> Router {
    Router::new()
        .route("/protected", get(protected))
        .route("/authorize", post(authorize))
    // .route("/test", post(index))
}

async fn index(client: Extension<RedisState>) -> response::Json<Value> {
    let mut conn = client
        .lock()
        .unwrap()
        .get_connection()
        .expect("failed to connect to Redis");
    let _: () = conn.set("test", "liqiang").unwrap();
    let rv: String = conn.get("test").unwrap();
    println!("{}", rv);
    response::Json(json!({ "result": "ok" }))

    //"欢迎来到中华诗词的世界"
}
