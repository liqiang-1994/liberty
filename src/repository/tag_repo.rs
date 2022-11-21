use crate::entity::{t_tag, prelude::TTag};
use crate::service::tag_service::{LibertyTag, Poem};
use sea_orm::{DbConn, DbErr, InsertResult};
use chrono::{Local, NaiveDateTime};
use sea_orm::ActiveValue::Set;
use sea_orm::EntityTrait;
use sea_orm::ActiveModelTrait;


pub async fn find_by_id(db: &DbConn, id: i32) -> Result<Option<t_tag::Model>, DbErr> {
    TTag::find_by_id(id).one(db).await
}

pub async fn save_tag(db: &DbConn, vo: LibertyTag) -> Result<t_tag::Model, DbErr> {
    let ta = t_tag::ActiveModel {
        id: Set(3),
        name: Set(Some(vo.name)),
        create_time: Set(Some(NaiveDateTime::from(Local::now().naive_local()))),
    };
    ta.insert(db).await
}

pub async fn update_tag(db: &DbConn, vo: LibertyTag) -> Result<t_tag::Model, DbErr> {
    let tag: t_tag::ActiveModel = TTag::find_by_id(vo.id.unwrap())
        .one(db)
        .await?
        .ok_or(DbErr::Custom("Cannot find post.".to_owned()))
        .map(Into::into)?;
    t_tag::ActiveModel {
        id: tag.id,
        name: Set(Some(vo.name)),
        create_time: Set(Some(NaiveDateTime::from(Local::now().naive_local())))
    }.update(db).await

}
