//use crate::config::Config;
//use actix_web::client::Client;
//use actix_web::middleware::Logger;
//use actix_web::{guard, web, App, HttpServer};
//use std::sync::Arc;
//
//pub mod config;
//mod proxy;
//pub struct Ela {
//    config: Arc<Config>,
//}
//
//impl Ela {
//    pub fn new() -> Self {
//        Self {
//            config: Arc::new(Config::load("./ela.toml")),
//        }
//    }
//
//    //    fn run(&self) -> std::io::Result<()> {
//    //        let arc_config = self.config.clone();
//    //        let listen_addr = (arc_config.ela.addr.as_str(), arc_config.ela.port);
//    //        let closure_config = self.config.clone();
//    //        HttpServer::new(move || {
//    //            let mut app = App::new()
//    //                .data(Client::new())
//    //                .data(closure_config.clone())
//    //                .wrap(Logger::default());
//    //
//    //            let arc = closure_config.clone();
//    //            for site in arc.sites.iter() {
//    //                let scope1 = web::scope("/").guard(guard::fn_guard(move |req| {
//    //                    req.headers()
//    //                        .get("host")
//    //                        .filter(|host| {
//    //                            site.domain
//    //                                .contains(&host.to_str().expect("cannot get host").to_string())
//    //                        })
//    //                        .is_some()
//    //                }));
//    //                //            if let Some(f) = site.proxy {
//    //                let scope2 = scope1.default_service(
//    //                    web::route().to_async(proxy::forward),
//    //                    //                    web::route().to_async(forward_with_target(&Url::parse(&f).unwrap())),
//    //                );
//    //                //            };
//    //                app = app.service(scope2);
//    //            }
//    //            app
//    //        })
//    //        .bind(listen_addr)?
//    //        .bind(("127.0.0.1", 7001))?
//    //        .system_exit()
//    //        .run()
//    //    }
//}
