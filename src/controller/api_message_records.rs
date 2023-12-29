
use salvo::prelude::*;
use crate::
    app::{Result, Vo}
;

#[endpoint(tags("apiMessageTestRecordsController"))]
pub async fn add(_req: &mut Request,_depot: &mut Depot) -> Result<Vo<String>> {
    todo!()
}

#[endpoint(tags("apiMessageTestRecordsController"))]
pub async fn get_api_id(_req: &mut Request,_depot: &mut Depot) -> Result<Vo<String>> {
    todo!()
}
