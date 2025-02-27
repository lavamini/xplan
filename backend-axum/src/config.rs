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
}

#[derive(Deserialize)]
pub struct Config {
    pub db: Db,
}

pub fn load_config(file_path: &str) -> Config {
    let mut file = match File::open(file_path) {
        Ok(f) => f,
        Err(e) => panic!("No such file {} exception: {}", file_path, e)
    };

    let mut str_val = String::new();
    match file.read_to_string(&mut str_val) {
        Ok(s) => s,
        Err(e) => panic!("Error reading file: {}", e)
    };

    let config: Config = toml::from_str(&str_val).unwrap();
    config
}
