use config::{Config, ConfigError, File};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[allow(unused)]
pub struct Environment {
    rust_log: String,
}

#[derive(Debug, Deserialize)]
#[allow(unused)]
pub struct Settings {
    pub env: Environment,
}

impl Settings {
    pub fn new() -> Result<Self, ConfigError> {
        // let run_mode = env::var("RUN_MODE").unwrap_or_else(|_| "development".into());

        let s = Config::builder()
            .add_source(File::with_name(".cargo/config"))
            .add_source(config::Environment::with_prefix("APP").separator("_"))
            .build()?;

        s.try_deserialize()
    }
}
