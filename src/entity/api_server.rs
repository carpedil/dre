//! `SeaORM` Entity. Generated by sea-orm-codegen 0.12.10

use salvo::oapi::ToSchema;
use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize,ToSchema)]
#[sea_orm(table_name = "api_server")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: i32,
    pub srv_name: Option<String>,
    pub default_hdr: Option<String>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_many = "super::api_message::Entity")]
    ApiMessage,
}

impl Related<super::api_message::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::ApiMessage.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
