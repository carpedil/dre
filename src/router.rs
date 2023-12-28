use salvo::prelude::*;

use crate::{app::*, controller::*};

pub async fn setup() -> Router {
    // create post table if not exists
    let conn = sea_orm::Database::connect("sqlite:./app.db?mode=rwc")
        .await
        .unwrap();
    let state = AppState { conn };

    let router = Router::new()
        .hoop(affix::inject(state))
        .path("api")
        .push(
            Router::with_path("amc")
                .post(api_message::add)
                .push(Router::with_path("<id>").get(api_message::get_by_id))
                .push(Router::with_path("<srv_id>").get(api_message::get_by_srv_id))
                .push(Router::with_path("update").post(api_message::update))
                .push(Router::with_path("hostname").post(api_message::hostname))
                .push(Router::with_path("state/update").post(api_message::state_update))
                .push(Router::with_path("script/execute").post(api_message::script_exec))
                .push(Router::with_path("validation").post(api_message::validation))
                .push(Router::with_path("del").delete(api_message::del)),
        )
        .push(
            Router::with_path("asc")
                .post(api_server::add)
                .get(api_server::list),
        )
        .push(
            Router::with_path("amd")
                .post(api_message_doc::add)
                .push(Router::with_path("<api_id>").get(api_message_doc::get_api_id))
                .push(Router::with_path("update/<id>").post(api_message_doc::update))
                .push(Router::with_path("del").delete(api_message_doc::del)),
        )
        .push(
            Router::with_path("amr")
                .post(api_message::add)
                .push(Router::with_path("<api_id>").get(api_message_records::get_api_id)),
        )
        .push(
            Router::with_path("report")
                .push(Router::with_path("statistic/count").get(api_message_report::statistic_count))
                .push(Router::with_path("summarize").get(api_message_report::summarize)),
        )
        .push(
            Router::with_path("ams")
                .post(api_message_script::add)
                .get(api_message_script::list),
        )
        .push(Router::with_path("static/<**>").get(StaticDir::new(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/static"
        ))));
    let doc = OpenApi::new("test api", "3.1.0").merge_router(&router);

    let router = router
        // .push(doc.into_router("/api-doc/openapi.json"))
        // .push(SwaggerUi::new("/api/api-doc/openapi.json").into_router("swagger-ui"));
        .unshift(doc.into_router("/api-doc/openapi.json"))
        .unshift(
            SwaggerUi::new("/api/api-doc/openapi.json")
                .title("Todos - SwaggerUI")
                .into_router("/swagger-ui"),
        );
        
    router
}
