use salvo::prelude::*;
use crate::
    app::{Result, Vo}
;

#[endpoint(tags("apiMessageReportController"))]
pub async fn statistic_count(_req: &mut Request,_depot: &mut Depot) -> Result<Vo<String>> {
    todo!()
}

#[endpoint(tags("apiMessageReportController"))]
pub async fn summarize(_req: &mut Request,_depot: &mut Depot) -> Result<Vo<String>> {
    todo!()
}
