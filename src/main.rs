use crate::config::Config;
use actix_web::{web, App, HttpServer};
use std::rc::Rc;

mod config;

fn main() -> std::io::Result<()> {
    let config = Config::load("ela.toml");
    let config_rc = Rc::new(config);
    let rc = config_rc.clone();
    HttpServer::new(move || App::new().service(web::resource("/").to(|| "hello world")))
        .bind((rc.ela.addr.as_str(), rc.ela.port))?
        .system_exit()
        .run();
    Ok(())
}
