use std::net::SocketAddr;

use axum::Router;
use config::{get_config, Config};
use rusqlite::params;

mod config;

#[tokio::main]
async fn main() {
    run().await;
}

#[derive(Debug)]
struct Person {
    id: i32,
    name: String,
}

async fn run() {
    #[cfg(debug)]
    dotenv::dotenv().ok();
    let config = get_config();

    database_example().unwrap();

    let app = Router::new().route("/", axum::routing::get(index));
    let addr = get_address(config);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

fn database_example() -> Result<(), rusqlite::Error> {
    let conn = rusqlite::Connection::open_in_memory()?;
    conn.execute(
        "CREATE TABLE person (
                id      INTEGER PRIMARY KEY,
                name    TEXT NOT NULL
            )",
        [],
    )
    .unwrap();
    let me = Person {
        id: 0,
        name: "John Smith".to_owned(),
    };
    conn.execute(
        "INSERT INTO person (id, name) VALUES (?1, ?2)",
        params![me.id, me.name],
    )?;
    let mut stmt = conn.prepare("SELECT id, name FROM person")?;

    let person_iter = stmt.query_map([], |row| {
        Ok(Person {
            id: row.get(0)?,
            name: row.get(1)?,
        })
    })?;

    for person in person_iter {
        println!("Found person {:#?}", person.unwrap());
    }

    Ok(())
}

fn get_address(config: impl AsRef<dyn Config>) -> SocketAddr {
    let config = config.as_ref();
    let port = config.port();
    config.message();
    ([0, 0, 0, 0], port).into()
}

async fn index() -> &'static str {
    //! Test with just a static "Hello, World!" string
    "Hello, World!"
}
