use salvo::http::StatusError;
use sea_orm::DatabaseConnection;

pub type Result<T> = std::result::Result<T, StatusError>;

#[derive(Clone, Debug)]
pub struct AppState {
    pub conn: DatabaseConnection,
}
