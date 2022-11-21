#![feature(once_cell)]
#![feature(generic_associated_types)]

mod comm;
mod db;
mod entity;
mod model;
mod repository;
mod service;
mod util;

use axum::http::StatusCode;
use axum::middleware::AddExtension;
use axum::response::IntoResponse;
use axum::routing::{get, post};
use axum::{response, Extension, Router};
use log::info;
use redis::{Client, Commands};
use serde_json::{json, Value};
use snowflake::{SnowflakeIdBucket, SnowflakeIdGenerator};
use std::net::SocketAddr;
use std::sync::{Arc, Mutex};

use crate::comm::RedisState;
use crate::db::db_conn;
//use crate::service::search_service::search_key;
use crate::service::storage_service::upload;
use crate::service::tag_service::{create_tag};
use crate::util::shutdown::shutdown_signal;
use sea_orm;
use sea_orm::DbConn;
use sqlx::{MySql, Pool};
use tower::{limit::ConcurrencyLimitLayer, ServiceBuilder};
use tracing_subscriber::util::SubscriberInitExt;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG)
        .with_test_writer()
        .init();
    let pool = db_conn().await;
    let redis_url = format!("redis://:{}@{}", "", "127.0.0.1:6379");
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
       // .nest("/search", tag_api())
        .nest("/tag", tag_api());

    let app = Router::new().nest("/api", api).layer(
        ServiceBuilder::new()
            .layer(ConcurrencyLimitLayer::new(10))
            .layer(Extension(pool))
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
        // .route("/upload", post(upload))
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
