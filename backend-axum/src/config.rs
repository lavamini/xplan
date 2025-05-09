use serde::Deserialize;

use std::fs::File;
use std::io::prelude::*;

#[derive(Deserialize)]
pub struct Db {
    pub host: String,
    pub port: u16,
    pub user: String,
    pub password: String,
    pub database: String,
    pub min_conns: u32,
    pub max_conns: u32,
}

#[derive(Deserialize)]
pub struct Config {
    pub db: Db,
}

pub fn load_config(file_path: &str) -> Config {
    tracing::debug!("loading config file ...");
    let mut file = match File::open(file_path) {
        Ok(f) => f,
        Err(err) => {
            tracing::error!("can't read config file: {}", err.to_string());
            std::process::exit(1);
        }
    };

    let mut str_val = String::new();
    match file.read_to_string(&mut str_val) {
        Ok(s) => s,
        Err(err) => {
            tracing::error!("can't read config file: {}", err.to_string());
            std::process::exit(1);
        }
    };

    let config: Config = toml::from_str(&str_val).unwrap();
    config
}
