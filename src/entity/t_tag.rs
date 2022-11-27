use chrono::{FixedOffset, Local};
use sea_orm::entity::prelude::*;
use sea_orm::{ActiveModelBehavior, ActiveModelTrait, ActiveValue};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "t_tag")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub name: Option<String>,
    pub create_time: Option<DateTime>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
