use actix_web::dev::RequestHead;
use actix_web::guard::Guard;
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::Read;
use std::path::Path;

#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    pub ela: Ela,
    pub sites: Vec<Site>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Ela {
    pub addr: String,
    pub port: u16,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Site {
    pub domain: String,
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

impl Guard for Site {
    fn check(&self, request: &RequestHead) -> bool {
        let host = request
            .headers
            .get("host")
            .map_or("", |value| value.to_str().unwrap_or(""));

        self.domain.eq(host)
    }
}
