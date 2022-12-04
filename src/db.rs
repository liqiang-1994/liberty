use crate::model::config::{Config, Postgres};
use sea_orm::{ConnectOptions, Database, DatabaseConnection};
use std::time::Duration;

pub async fn db_conn(cfg: &Postgres) -> DatabaseConnection {
    let url = format!(
        "{}://{}:{}@{}:{}/{}",
        cfg.driver, cfg.username, cfg.password, cfg.url, cfg.port, cfg.db
    );
    let mut opt = ConnectOptions::new(url);
    opt.max_connections(100)
        .min_connections(5)
        .connect_timeout(Duration::from_secs(30))
        .acquire_timeout(Duration::from_secs(30))
        .idle_timeout(Duration::from_secs(8))
        .max_lifetime(Duration::from_secs(10))
        .sqlx_logging(true)
        .sqlx_logging_level(log::LevelFilter::Info);
    let db = Database::connect(opt)
        .await
        .expect("Database connection failed");
    db
}
