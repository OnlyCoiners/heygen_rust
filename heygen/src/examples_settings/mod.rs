use dotenvy::dotenv;
use std::env;

lazy_static::lazy_static! {
    pub static ref SETTINGS: Settings = Settings::new();
}

#[derive(Debug, PartialEq, Clone)]
pub struct Settings {
    pub api_heygen_token: String,
}

impl Settings {
    pub fn new() -> Self {
        if let Err(_) = dotenv() {
            tracing::warn!(
                "No .env file and env variables should be provided by your hosting service"
            );
        }

        Settings {
            api_heygen_token: env::var("API_HEYGEN_TOKEN").expect("API_HEYGEN_TOKEN must be set"),
        }
    }
}
