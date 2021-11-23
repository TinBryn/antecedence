pub trait Config {
    fn port_message_action(&self, port: u16);
}

pub struct DebugConfig;

impl Config for DebugConfig {
    fn port_message_action(&self, port: u16) {
        println!("serve on http://localhost:{}", port)
    }
}

pub struct HerokuConfig;

impl Config for HerokuConfig {
    fn port_message_action(&self, port: u16) {
        println!("serve on https://antecedence.herokuapp.com:{}", port)
    }
}

pub fn get_config() -> Box<dyn Config> {
    if let Ok(value) = std::env::var("CONFIG") {
        if value == "heroku" {
            return Box::new(HerokuConfig);
        }
    }
    Box::new(DebugConfig)
}
