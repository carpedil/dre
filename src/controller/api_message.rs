use crate::{
    app::{AppState, Result},
    entity::prelude::{ApiMessage, ApiMessageModel, ApiServerModel},
};
use salvo::prelude::*;
use sea_orm::EntityTrait;

#[handler]
pub async fn list(_req: &mut Request, depot: &mut Depot) -> Result<Json<Vec<ApiMessageModel>>> {
    let state = depot
        .obtain::<AppState>()
        .map_err(|_| StatusError::internal_server_error())?;

    let res = ApiMessage::find()
        .all(&state.conn)
        .await
        .map_err(|_| StatusError::internal_server_error())?;
    Ok(Json(res))
}

#[handler]
pub async fn add(req: &mut Request, _depot: &mut Depot) -> Result<String> {
    let am = req.parse_json::<ApiServerModel>().await.unwrap();
    tracing::info!("{:?}",am);
    Ok("".to_string())
}

#[handler]
pub async fn del(req: &mut Request, depot: &mut Depot) -> Result<String> {
    Ok("".to_string())
}

#[handler]
pub async fn validation(req: &mut Request, depot: &mut Depot) -> Result<()> {
    Ok(())
}

#[handler]
pub async fn get_by_id(req: &mut Request, depot: &mut Depot) -> Result<()> {
    Ok(())
}

#[handler]
pub async fn get_by_srv_id(req: &mut Request, depot: &mut Depot) -> Result<()> {
    Ok(())
}
