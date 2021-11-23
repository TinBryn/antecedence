use std::net::SocketAddr;

use axum::Router;

trait Config {
    fn port_message_action(&self, port: u16);
}

struct DebugConfig;

impl Config for DebugConfig {
    fn port_message_action(&self, port: u16) {
        println!("serve on http://localhost:{}", port)
    }
}

struct HerokuConfig;

impl Config for HerokuConfig {
    fn port_message_action(&self, port: u16) {
        println!("serve on https://antecedence.herokuapp.com:{}", port)
    }
}

#[tokio::main]
async fn main() {
    make_and_run_app().await;
}

async fn make_and_run_app() {
    let config = get_config();
    let app = Router::new().route("/", axum::routing::get(index));
    let addr = get_address(config.as_ref());
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

fn get_config() -> Box<dyn Config> {
    if let Ok(value) = std::env::var("CONFIG") {
        if value == "heroku" {
            return Box::new(HerokuConfig);
        }
    }
    Box::new(DebugConfig)
}

fn get_address(config: &dyn Config) -> SocketAddr {
    let port = std::env::var("PORT")
        .unwrap_or_else(|_| "8080".to_owned())
        .parse()
        .expect("PORT must be a number");
    config.port_message_action(port);
    ([0, 0, 0, 0], port).into()
}

/// Test with just a static "Hello, World!" string
async fn index() -> &'static str {
    "Hello, World!"
}
