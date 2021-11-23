pub trait Config {
    fn message(&self, port: u16);
    fn port(&self) -> u16;
}

pub struct DebugConfig;

impl Config for DebugConfig {
    fn message(&self, port: u16) {
        println!("serve on http://localhost:{}", port)
    }
    fn port(&self) -> u16 {
        8080
    }
}

pub struct HerokuConfig {
    port: u16,
}

impl HerokuConfig {
    pub fn new() -> Self {
        let port: u16 = std::env::var("PORT")
            .expect("Heroku should have PORT variable")
            .parse()
            .expect("PORT should be a number");
        HerokuConfig { port }
    }
}

impl Config for HerokuConfig {
    fn message(&self, port: u16) {
        println!("serve on https://antecedence.herokuapp.com:{}", port)
    }
    fn port(&self) -> u16 {
        self.port
    }
}

pub fn get_config() -> Box<dyn Config> {
    if let Ok(value) = std::env::var("CONFIG") {
        if value == "heroku" {
            return Box::new(HerokuConfig::new());
        }
    }
    Box::new(DebugConfig)
}
