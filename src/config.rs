use std::path::Path;

mod heroku;
use heroku::config as heroku_cfg;
mod debug;
use debug::config as debug_cfg;
mod render;
use render::config as render_cfg;

/// What a configuration will need to do or provide
pub trait Config {
    /// Provide some message to the user, such as a link to the server
    fn message(&self);
    /// Get the port number to use, either from environment variable or hard coded
    fn port(&self) -> u16;
    /// The url to get a database from
    fn database_url(&self) -> &Path;
}

pub fn get_config() -> Box<dyn Config> {
    match dbg!(std::env::var("CONFIG").as_deref()) {
        Ok(value) => match value {
            "heroku" => heroku_cfg(),
            "render" => render_cfg(),
            _ => debug_cfg(),
        },
        Err(_) => debug_cfg(),
    }
}
