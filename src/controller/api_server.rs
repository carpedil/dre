use salvo::prelude::*;
use sea_orm::EntityTrait;
use crate::{
    app::{Result, AppState, Vo}, entity::prelude::{ApiServerModel, ApiServer}}
;

#[endpoint(tags("apiServerController"))]
pub async fn add(_req: &mut Request,_depot: &mut Depot) -> Result<Vo<String>> {
    Ok(Json(Vo::new(String::new())))
}

#[endpoint(tags("apiServerController"))]
pub async fn list(_req: &mut Request,depot: &mut Depot) -> Result<Vo<Vec<ApiServerModel>>> {
    let state = depot.obtain::<AppState>()
    .map_err(|_| StatusError::internal_server_error())?;

    let data = ApiServer::find().all(&state.conn).await
    .map_err(|_| StatusError::internal_server_error())?;

    Ok(Json(Vo::new(data)))
}
