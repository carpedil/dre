use app::*;
use controller::api_message;
use salvo::prelude::*;

mod app;
mod controller;
mod entity;

#[tokio::main]
async fn main() {
    std::env::set_var("RUST_LOG", "debug");
    tracing_subscriber::fmt::init();
    let server_url = "0.0.0.0:5800";

    // create post table if not exists
    let conn = sea_orm::Database::connect("sqlite:./app.db?mode=rwc")
        .await
        .unwrap();
    let state = AppState { conn };

    println!("Starting server at {server_url}");
    let router = Router::new()
        .hoop(affix::inject(state))
        .post(api_message::add)
        .get(api_message::list)
        // .push(Router::with_path("new").get(new))
        // .push(Router::with_path("<id>").get(edit).post(update))
        // .push(Router::with_path("delete/<id>").post(delete))
        .push(Router::with_path("static/<**>").get(StaticDir::new(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/static"
        ))));

    let acceptor = TcpListener::new(&server_url).bind().await;
    Server::new(acceptor).serve(router).await;
}
