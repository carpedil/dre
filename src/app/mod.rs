use salvo::{http::StatusError, oapi::ToSchema};
use sea_orm::DatabaseConnection;
use serde::{Serialize, Deserialize};

pub type Result<ResultData> = std::result::Result<ResultData, StatusError>;

#[derive(Clone, Debug)]
pub struct AppState {
    pub conn: DatabaseConnection,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, ToSchema)]
pub struct ResultData<T: salvo::prelude::ToSchema + 'static> {
    code: u32,
    message:String,
    data: T
}

impl<T: salvo::prelude::ToSchema> ResultData<T> {
    pub fn new(data :T) -> Self {
        Self {
            code: 200,
            message: String::from("ok"),
            data
        }
    }
}