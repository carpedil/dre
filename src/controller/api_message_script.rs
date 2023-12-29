use salvo::prelude::*;
use crate::
    app::{Result, Vo}
;

#[endpoint(tags("apiMessageScriptController"))]
pub async fn add(_req: &mut Request,_depot: &mut Depot) -> Result<Vo<String>> {
    todo!()
}

#[endpoint(tags("apiMessageScriptController"))]
pub async fn list(_req: &mut Request,_depot: &mut Depot) -> Result<Vo<String>> {
    todo!()
}
