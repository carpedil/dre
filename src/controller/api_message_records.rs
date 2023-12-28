
use salvo::prelude::*;
use crate::
    app::Result
;

#[endpoint(tags("apiMessageTestRecordsController"))]
pub async fn add(_req: &mut Request,_depot: &mut Depot) -> Result<()> {
    Ok(())
}

#[endpoint(tags("apiMessageTestRecordsController"))]
pub async fn get_api_id(_req: &mut Request,_depot: &mut Depot) -> Result<()> {
    Ok(())
}
