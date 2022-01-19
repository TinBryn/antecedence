use std::path::Path;

use super::Config;

/**
 * Configuration settings for reading Heroku's environment variables
 *
 * - `PORT` is read to get the port to bind to
 * - `DATABASE_URL` is read
 */
pub struct Heroku {
    port: u16,
    database_url: String,
}

pub fn config() -> Box<dyn Config> {
    let port: u16 = std::env::var("PORT")
        .expect("Heroku should have PORT variable")
        .parse()
        .expect("PORT should be a number");
    let database_url = std::env::var("DATABASE_URL").unwrap_or_else(|_| "antecedence.db".into());
    Box::new(Heroku { port, database_url })
}

impl Config for Heroku {
    fn message(&self) {
        println!("serve on https://antecedence.herokuapp.com:{}", self.port)
    }
    fn port(&self) -> u16 {
        self.port
    }
    fn database_url(&self) -> &Path {
        self.database_url.as_ref()
    }
}
