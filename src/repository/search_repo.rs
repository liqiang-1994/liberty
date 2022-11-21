use crate::model::search::{KeySearch, PoemResp};
use crate::service::tag_service::Poem;
use axum::Json;
use sqlx::mysql::MySqlQueryResult;
use sqlx::{Error, MySql, Pool};

// pub async fn search_by_key(pool: &Pool<MySql>, key : KeySearch) -> Result<Vec<PoemResp>, sqlx::Error> {
//     let result = sqlx::query_as!(PoemResp,
//         "
//         select * from t_poetry where author like ? or content like ? or rhythmic like ?
//         ", key.key, key.key, key.key
//     ).fetch_all(&*pool)
//         .await;
//     result
//
// }

// pub async fn count_by_key(pool: &Pool<MySql>, key : KeySearch) -> i64 {
//     let result = sqlx::query_as!(PoemResp,
//         "
//         select * from t_poetry where author like ? or or content like ? or rhythmic like ?
//         ", key.key, key.key, key.key
//     ).fetch_all(&*pool)
//         .await;
//     result
//
//     // sqlx::query_as::<_, PoemResp>(
//     //     r#"
//     //     select author, content from t_poetry where author limit 1"#
//     // ).fetch_one(&*pool).await
//
// }
