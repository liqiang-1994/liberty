use log::info;
use snowflake::SnowflakeIdBucket;

pub fn id_gen() -> i64 {
    let mut id_bucket = SnowflakeIdBucket::new(1, 1);
    let id = id_bucket.get_id();
    info!("id generate {}", id);
    id
}
