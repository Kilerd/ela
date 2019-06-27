use std::rc::Rc;
use std::sync::Arc;

use actix_web::{client::Client, guard, web, App, HttpServer};

use crate::config::Config;

mod config;
mod proxy;

fn main() -> std::io::Result<()> {
    let config = Arc::new(Config::load("ela.toml"));
    let rc = config.clone();

    println!("Ela is binding on {}:{}", rc.ela.addr, rc.ela.port);

    for site in &rc.sites {
        if let Some(proxy) = &site.proxy {
            println!("proxy {} to {}", &site.domain, proxy);
        }
    }

    HttpServer::new(move || {
        let mut app = App::new().data(config.clone()).data(Client::new());

        let sites = config.clone().sites.clone();

        for site in sites {
            if site.proxy.is_some() {
                let host = site.domain.clone().as_str();
                app = app.service(
                    web::resource("/*")
                        .guard(guard::Any(site))
                        .to_async(proxy::forward),
                )
            }
        }
        app
    })
    .bind((rc.ela.addr.as_str(), rc.ela.port))?
    .system_exit()
    .run();
    Ok(())
}
