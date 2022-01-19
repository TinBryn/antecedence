use std::path::{Path, PathBuf};

use super::Config;

/**
 * A default configuration for when the "CONFIG" environment variable is either not present
 * or its value is not recognised.
 */
pub struct Debug {
    url: PathBuf,
}

pub fn config() -> Box<dyn Config> {
    let url = ["sqlite:///", "app.db"].iter().collect();
    println!("sqlite database: {:?}", url);
    Box::new(Debug { url })
}

impl Debug {
    const PORT: u16 = 8080;
}

impl Config for Debug {
    fn message(&self) {
        println!("serve on http://localhost:{}", Self::PORT)
    }
    fn port(&self) -> u16 {
        Self::PORT
    }
    fn database_url(&self) -> &Path {
        &self.url
    }
}
