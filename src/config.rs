use std::{fs, io};

use serde::{Deserialize, Serialize};
use toml;

extern crate dirs;
extern crate serde;

const CONFIG_FILENAME: &'static str = "urun.toml";

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Config {
    pub history: Vec<String>,
}

impl Default for Config {
    fn default() -> Self {
        Config { history: vec![] }
    }
}

impl Config {
    pub fn from_disc() -> io::Result<Self> {
        let mut config_path = dirs::config_dir().unwrap();
        config_path.push(CONFIG_FILENAME);

        let contents: String = std::fs::read_to_string(config_path)?;

        return toml::from_str(&contents).or_else(|_| {
            eprintln!("Corrupt config file! Proceeding with default.");
            Ok(Default::default())
        });
    }

    pub fn to_disc(&self) -> io::Result<()> {
        let serialized = toml::to_string(&self).unwrap();

        let mut config_path = dirs::config_dir().unwrap();
        config_path.push(CONFIG_FILENAME);

        return fs::write(config_path, serialized);
    }
}
