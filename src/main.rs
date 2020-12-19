#[macro_use]
extern crate nonblock_logger;
#[macro_use]
extern crate sqlx;
#[macro_use]
extern crate async_trait;
#[macro_use]
extern crate serde;

use actix_web::{get, web, Responder, HttpServer, App};
use sqlx::MySqlPool;
use sqlx::mysql::MySqlPoolOptions;
use std::env;
use anyhow::Result;
use crate::config::{Opt, Config};

pub mod tag;
pub mod config;
pub mod state;
pub mod response;

#[get("/{id}/{name}")]
async fn index(web::Path((id, name)):web::Path<(u32, String)>) -> impl Responder {
    std::format!("hello {} id:{}", id, name)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let(_handle, opt) = Opt::parse_from_args();
    let state = Config::parse_from_file(&opt.config).into_state().await;

    HttpServer::new(move|| {
        App::new()
            .data(state.clone())
            .app_data(state.clone())
            //.data(pool.clone())
            .service(web::scope("/tag").configure(tag::route::init))
            .service(index)
            //.route("/", web::get().to(index))
            //.configure()
    })
        .keep_alive(300)
        .bind("127.0.0.1:8080")?
        .run()
        .await
}
