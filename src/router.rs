use salvo::{prelude::*, serve_static::static_embed};
use rust_embed::RustEmbed;
use crate::{app::*, controller::*,ws::{self}};

#[derive(RustEmbed)]
#[folder = "src/templates"]
struct Assets;

pub async fn setup() -> Router {
    // create post table if not exists
    let conn = sea_orm::Database::connect("sqlite:./app.db?mode=rwc")
        .await
        .unwrap();
    let state = AppState { conn };

    let router = Router::new()
        .hoop(affix::inject(state))
        // websocket support
        .push(Router::with_path("ws").goal(ws::connect))
        .push(
            Router::new().path("api")
            .push(
                Router::with_path("amc")
                    .push(Router::with_path("add").post(api_message::add))
                    .push(Router::with_path("list").post(api_message::list))
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
                    .push(Router::with_path("add").post(api_server::add))
                    .push(Router::with_path("list").get(api_server::list))
            )
            .push(
                Router::with_path("amd")
                    .push(Router::with_path("add").post(api_message_doc::add))
                    .push(Router::with_path("<api_id>").get(api_message_doc::get_api_id))
                    .push(Router::with_path("update/<id>").post(api_message_doc::update))
                    .push(Router::with_path("del").delete(api_message_doc::del)),
            )
            .push(
                Router::with_path("amr")
                    .push(Router::with_path("add").post(api_message_records::add))
                    .push(Router::with_path("<api_id>").get(api_message_records::get_api_id)),
            )
            .push(
                Router::with_path("report")
                    .push(Router::with_path("statistic/count").get(api_message_report::statistic_count))
                    .push(Router::with_path("record/summary").get(api_message_report::summarize)),
            )
            .push(
                Router::with_path("ams")
                    .post(api_message_script::add)
                    .get(api_message_script::list),
            )
        )
        .push(Router::with_path("<**>").get(static_embed::<Assets>().fallback("index.html")));

    let doc = OpenApi::new("test api", "3.1.0").merge_router(&router);

    let router = router
        .unshift(doc.into_router("/api-doc/openapi.json"))
        .unshift(
            SwaggerUi::new("/api-doc/openapi.json")
                .title("Dreams-api - SwaggerUI")
                .into_router("/swagger-ui"),
        );

    router
}
