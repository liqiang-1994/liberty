use crate::model::response::Response;
use crate::model::search::KeySearch;
use axum::extract::Extension;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::Json;
use log::info;
use std::fmt::Debug;
//use crate::repository::search_repo::search_by_key;

// pub async fn search_key(Json(req): Json<KeySearch>, Extension(pool): Extension<Pool<MySql>>) -> impl IntoResponse {
//     info!("search api req:{:#?}",req);
//     // let result = match req.search_type  {
//     //     "1" =>,
//     //
//     //     _ => {}
//     // }
//     match search_by_key(&pool, req).await {
//         Ok(result) => {
//             println!("{:#?}", result);
//             (StatusCode::OK, Json(Response::new(Option::None).with_data(result)))
//         }
//         Err(e) => {
//             println!("{:#?}", e);
//             (StatusCode::OK, Json(Response::new(Option::None).with_msg("err")))
//         }
//     }
//
// }
