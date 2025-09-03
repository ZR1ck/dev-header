use std::fs;

use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct Config {
    pub max_header_len: u16,
    pub max_title_len: u16,
    pub header_len: u16,
    pub space_height: u8,
}

impl Config {
    pub fn new() -> Self {
        let data = fs::read_to_string("config.json").unwrap();
        serde_json::from_str::<Config>(&data).unwrap()
    }
}
