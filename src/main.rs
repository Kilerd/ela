use actix_web::client::Client;
use actix_web::{HttpServer, App, web, HttpResponse, HttpRequest, Error};
use actix_web::middleware::Logger;
use futures::Future;
use url::Url;
use futures::stream::Stream;

fn forward(req: HttpRequest, payload: web::Payload, client: web::Data<Client>, target: web::Data<Url>) -> impl Future<Item=HttpResponse, Error=Error> {
    let mut target_host = target.get_ref().clone();
    target_host.set_path(req.uri().path());
    target_host.set_query(req.uri().query());

    let forwarded_req = client.request_from(target_host.as_str(), req.head());
    forwarded_req
        .send_stream(payload)
        .map_err(Error::from)
        .map(|res| {
            let mut client_resp = HttpResponse::build(res.status());
            for (header_name, header_value) in res.headers().iter().filter(|(h, _)| *h != "connection") {
                client_resp.header(header_name.clone(), header_value.clone());
            }
            client_resp.streaming(res)
        })
}

fn main()-> std::io::Result<()> {
    let forward_url = Url::parse("http://127.0.0.1:8000").unwrap();
    HttpServer::new(move || {
        App::new()
            .data(Client::new())
            .data(forward_url.clone())
            .wrap(Logger::default())
            .default_service(web::route().to_async(forward))
    })
        .bind("127.0.0.1:7777")?
        .system_exit()
        .run()
}
