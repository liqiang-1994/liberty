use redis::Client;
use std::sync::{Arc, Mutex};

pub type RedisState = Arc<Mutex<Client>>;
