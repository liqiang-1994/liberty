use crate::config::Config;

pub type SqlPool = sqlx::MySqlPool;
pub type PoolOptions = sqlx::mysql::MySqlPoolOptions;

#[derive(Clone)]
pub struct State {
    pub config: Config,
    pub sql: SqlPool,
}

pub type AppStateRaw = std::sync::Arc<State>;
pub type AppState = actix_web::web::Data<AppStateRaw>;