use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::Read;
use std::path::Path;

#[derive(Serialize, Deserialize)]
pub struct Config {
    pub ela: Ela,
    pub sites: Vec<Site>,
}

#[derive(Serialize, Deserialize)]
pub struct Ela {
    pub addr: String,
    pub port: u16,
}
#[derive(Serialize, Deserialize)]
pub struct Site {
    pub domain: Vec<String>,
    pub hsts: Option<bool>,
    pub auto_ssl: Option<bool>,
    pub root: Option<String>,
    pub proxy: Option<String>,
}

impl Config {
    pub fn load(path: impl AsRef<Path>) -> Self {
        let mut file = File::open(path).expect("cant open file");
        let mut content = String::new();
        file.read_to_string(&mut content);

        toml::from_str(&String::from(content)).unwrap()
    }
}
