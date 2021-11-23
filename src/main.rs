use std::net::SocketAddr;

use axum::Router;

#[tokio::main]
async fn main() {
    let app = Router::new().route("/", axum::routing::get(index));

    let port = std::env::var("PORT")
        .unwrap_or_else(|_| "8080".to_owned())
        .parse()
        .expect("PORT must be a number");

    let addr = SocketAddr::from(([0,0,0,0], port));
    println!("serve from http://localhost:{}", port);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

/// Test with just a static "Hello, World!" string
async fn index() -> &'static str {
    "Hello, World!"
}
