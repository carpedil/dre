use salvo::{http::StatusError, oapi::ToSchema, writing::Json};
use sea_orm::DatabaseConnection;
use serde::{Serialize, Deserialize};

pub type Result<T> = std::result::Result<Json<T>, StatusError>;

#[derive(Clone, Debug)]
pub struct AppState {
    pub conn: DatabaseConnection,
}

#[derive(Clone, Debug, Serialize, Deserialize, ToSchema)]
pub struct Vo<T: salvo::prelude::ToSchema + 'static> {
    code: u32,
    message:String,
    data: T
}

impl<T: salvo::prelude::ToSchema> Vo<T> {
    pub fn new(data :T) -> Self {
        Self {
            code: 200,
            message: String::from("ok"),
            data
        }
    }
}