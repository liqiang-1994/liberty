//! `SeaORM` Entity. Generated by sea-orm-codegen 0.10.3

use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq)]
#[sea_orm(table_name = "t_user")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: i64,
    #[sea_orm(unique)]
    pub uid: String,
    pub user_name: String,
    pub address: Option<String>,
    #[sea_orm(unique)]
    pub phone: Option<String>,
    pub gender: Option<i16>,
    pub signature: Option<String>,
    pub avatar: Option<String>,
    pub status: Option<i64>,
    pub follow: Option<i64>,
    pub watch: Option<i64>,
    pub up: Option<i64>,
    pub login_time: Option<DateTime>,
    pub login_type: Option<i64>,
    pub create_time: Option<DateTime>,
    pub update_time: Option<DateTime>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
