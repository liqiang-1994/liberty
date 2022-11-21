use std::sync::{Arc, Mutex};
use redis::Client;

pub type RedisState = Arc<Mutex<Client>>;