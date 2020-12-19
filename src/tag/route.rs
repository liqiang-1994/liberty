use actix_web::{delete, get, post, put, web, HttpResponse, Responder};
use sqlx::MySqlPool;
use crate::tag::model::Tag;
use crate::state::AppState;
use crate::tag::dao::ITag;
use crate::response::ResponseEntity;

#[get("/tag/{id}")]
//async fn find_all(form: web::Json<Tag>, state: AppState) -> impl Responder {
async fn find_all(web::Path((id)):web::Path<(i64)>, state: AppState) -> impl Responder {
    //let form = form.into_inner();
    let form = Tag{
        id: id,
        name: "x".to_string(),
        description: "y".to_string(),
        create_uid: 0,
    };
    match state.add_tag(&form).await {
        Ok(res) => {
            info!("add tag {:?} res:{}", form, res);
            ResponseEntity::new().with_msg("ok").with_data(res)
        }
        Err(e) => {
            error!("add tag {:?} error:{}", form, e);
            ResponseEntity::new().code(400).with_msg(e.to_string())
        }
    }
}

pub fn init(cfg: &mut web::ServiceConfig) {
    cfg.service(find_all);
}