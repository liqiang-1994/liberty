use crate::repository::tag_repo::{save_tag, update_tag};
use axum::extract::{Extension, Query};
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::Json;
use sea_orm::{DatabaseConnection, DbConn};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;
use std::fs::File;
use std::io::Read;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct LibertyTag {
    pub id: Option<i32>,
    pub name: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CreateResp {
    pub ok: bool,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Poem {
    pub riddle: String,
    pub answer: String,
}

pub async fn create_tag(
    Extension(ref conn): Extension<DatabaseConnection>,
    Json(req): Json<LibertyTag>,
) -> impl IntoResponse {
    println!("{:#?}", req);
    match req.id {
        Some(v) => match update_tag(conn, req).await {
            Ok(result) => {
                println!("{:#?}", result);
                (StatusCode::OK, Json(CreateResp { ok: true }))
            }
            Err(e) => {
                println!("{:#?}", e);
                (StatusCode::OK, Json(CreateResp { ok: true }))
            }
        },
        None => match save_tag(conn, req).await {
            Ok(result) => {
                println!("{:#?}", result);
                (StatusCode::OK, Json(CreateResp { ok: true }))
            }
            Err(e) => {
                println!("{:#?}", e);
                (StatusCode::OK, Json(CreateResp { ok: true }))
            }
        },
    }
}

// pub async fn parse_poem(
//     Query(params): Query<HashMap<String, String>>,
//     Extension(pool): Extension<&DbConn>,
// ) -> impl IntoResponse {
//     let mut url = params.get("url").unwrap();
//     let mut file = File::open(&url).unwrap();
//     let json: Vec<Poem> = serde_json::from_reader(file).unwrap();
//     match save_poem(&pool, json).await {
//         Ok(result) => {
//             println!("{:#?}", result);
//             (StatusCode::OK, Json(CreateResp { ok: true }))
//         }
//         Err(e) => {
//             println!("{:#?}", e);
//             (StatusCode::OK, Json(CreateResp { ok: true }))
//         }
//     }
// }
