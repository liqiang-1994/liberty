use sea_orm::{ConnectOptions, Database, DatabaseConnection};
use sqlx::mysql::MySqlPoolOptions;
use sqlx::{MySql, Pool};
use std::time::Duration;

pub async fn db_conn() -> DatabaseConnection {
    let mut opt =
        ConnectOptions::new("postgresql://user:pass@host/db".to_owned());
    opt.max_connections(100)
        .min_connections(5)
        .connect_timeout(Duration::from_secs(8))
        .acquire_timeout(Duration::from_secs(8))
        .idle_timeout(Duration::from_secs(8))
        .max_lifetime(Duration::from_secs(10))
        .sqlx_logging(true)
        .sqlx_logging_level(log::LevelFilter::Info);
    let db = Database::connect(opt)
        .await
        .expect("Database connection failed");
    db
}
