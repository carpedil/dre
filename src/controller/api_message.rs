use crate::{
    app::{AppState, Result, Vo},
    entity::prelude::{ApiMessage, ApiMessageModel, ApiServerModel},
};
use salvo::prelude::*;
use sea_orm::EntityTrait;

#[endpoint(tags("apiMessageController"))]
pub async fn list(_req: &mut Request, depot: &mut Depot) -> Result<Vo<Vec<ApiMessageModel>>> {
    let state = depot
        .obtain::<AppState>()
        .map_err(|_| StatusError::internal_server_error())?;

    let res = ApiMessage::find()
        .all(&state.conn)
        .await
        .map_err(|_| StatusError::internal_server_error())?;
    Ok(Json(Vo::new(res)))
}

#[endpoint(tags("apiMessageController"))]
pub async fn add(req: &mut Request, _depot: &mut Depot) -> Result<Vo<String>> {
    let am = req.parse_json::<ApiServerModel>().await.unwrap();
    tracing::info!("{:?}",am);
    Ok(Json(Vo::new(String::new())))
}

#[endpoint(tags("apiMessageController"))]
pub async fn del(_req: &mut Request, _depot: &mut Depot) -> Result<Vo<String>> {
    todo!()
}

#[endpoint(tags("apiMessageController"))]
pub async fn validation(_req: &mut Request, _depot: &mut Depot) ->Result<Vo<String>> {
    todo!()
}

#[endpoint(tags("apiMessageController"))]
pub async fn get_by_id(_req: &mut Request, _depot: &mut Depot) -> Result<Vo<String>> {
    todo!()
}

#[endpoint(tags("apiMessageController"))]
pub async fn get_by_srv_id(_req: &mut Request, _depot: &mut Depot) -> Result<Vo<String>> {
    todo!()
}

#[endpoint(tags("apiMessageController"))]
pub async fn update(_req: &mut Request, _depot: &mut Depot) -> Result<Vo<String>> {
    todo!()
}


#[endpoint(tags("apiMessageController"))]
pub async fn hostname(_req: &mut Request,_depot: &mut Depot) -> Result<Vo<String>> {
    todo!()
}

#[endpoint(tags("apiMessageController"))]
pub async fn state_update(_req: &mut Request,_depot: &mut Depot) -> Result<Vo<String>> {
    todo!()
}


#[endpoint(tags("apiMessageController"))]
pub async fn script_exec(_req: &mut Request,_depot: &mut Depot) -> Result<Vo<String>> {
    todo!()
}

