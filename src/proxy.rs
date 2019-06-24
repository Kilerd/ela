use actix_web::client::Client;
use actix_web::{web, Error, HttpRequest, HttpResponse};
use futures::Future;
use url::Url;

pub fn forward(
    req: HttpRequest,
    payload: web::Payload,
) -> impl Future<Item = HttpResponse, Error = Error> {
    let mut target_host = Url::parse("http://127.0.0.1:100").unwrap();
    target_host.set_path(req.uri().path());
    target_host.set_query(req.uri().query());

    let forwarded_req = Client::new().request_from(target_host.as_str(), req.head());
    forwarded_req
        .send_stream(payload)
        .map_err(Error::from)
        .map(|res| {
            let mut client_resp = HttpResponse::build(res.status());
            for (header_name, header_value) in
                res.headers().iter().filter(|(h, _)| *h != "connection")
            {
                client_resp.header(header_name.clone(), header_value.clone());
            }
            client_resp.streaming(res)
        })
}
