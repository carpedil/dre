
use salvo::prelude::*;

mod app;
mod controller;
mod entity;
mod router;
mod ws;

#[tokio::main]
async fn main() {
    std::env::set_var("RUST_LOG", "debug");
    tracing_subscriber::fmt::init();
    let server_url = "0.0.0.0:8090";

    tracing::info!("Starting server at {server_url}");
   
    let router = router::setup().await;
    let acceptor = TcpListener::new(&server_url).bind().await;
    Server::new(acceptor).serve(router).await;
}
