use salvo::prelude::*;
use crate::
    app::Result
;

#[endpoint(tags("apiMessageScriptController"))]
pub async fn add(_req: &mut Request,_depot: &mut Depot) -> Result<()> {
    Ok(())
}

#[endpoint(tags("apiMessageScriptController"))]
pub async fn list(_req: &mut Request,_depot: &mut Depot) -> Result<()> {
    Ok(())
}
