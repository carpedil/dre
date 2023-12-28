//! `SeaORM` Entity. Generated by sea-orm-codegen 0.12.10

use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize)]
#[sea_orm(table_name = "api_message_test_record")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: i32,
    pub api_id: Option<i32>,
    pub api_content: Option<String>,
    pub result: Option<String>,
    pub callback_msg1: Option<String>,
    pub callback_msg2: Option<String>,
    pub comment: Option<String>,
    pub created_at: Option<String>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::api_message::Entity",
        from = "Column::ApiId",
        to = "super::api_message::Column::Id",
        on_update = "NoAction",
        on_delete = "NoAction"
    )]
    ApiMessage,
}

impl Related<super::api_message::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::ApiMessage.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
