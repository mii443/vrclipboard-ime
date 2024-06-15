use std::{fs::File, io::Read};

use serde_derive::Deserialize;
use anyhow::Result;

#[derive(Debug, Clone, Deserialize)]
pub struct Config {
    pub prefix: String,
    pub split: String,
    pub command: String,
}

impl Config {
    pub fn load() -> Result<Config> {
        let mut file = File::open("config.yaml")?;
        let mut contents = String::new();
        file.read_to_string(&mut contents)?;
        let config: Config = serde_yaml::from_str(&contents)?;
        Ok(config)
    }
}
