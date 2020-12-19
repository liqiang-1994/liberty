
#[derive(Serialize, Deserialize, Debug)]
pub struct Tag {
    pub id: i64,
    pub name: String,
    pub description: String,
    pub create_uid: i64,
    // pub create_dt: chrono::NaiveDateTime,
    // pub update_dt: chrono::NaiveDateTime
}