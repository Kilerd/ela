use crate::config::{Config, Site};
use actix_web::client::Client;
use actix_web::{web, Error, HttpRequest, HttpResponse};
use futures::Future;
use std::sync::Arc;
use url::Url;

pub fn forward(
    req: HttpRequest,
    config: web::Data<Arc<Config>>,
    client: web::Data<Client>,
    payload: web::Payload,
) -> impl Future<Item = HttpResponse, Error = Error> {
    let sites: Vec<&Site> = config
        .sites
        .iter()
        .filter(|&site| {
            site.domain.eq(req
                .headers()
                .get("host")
                .map(|value| value.to_str().unwrap_or(""))
                .unwrap_or(""))
        })
        .collect();

    let target_site = sites.get(0).expect("can not found proxy with host");

    let mut target_host = target_site
        .proxy
        .as_ref()
        .map(|proxy| Url::parse(proxy).expect(" cannot format as url"))
        .expect("proxy field must be set");

    target_host.set_path(req.uri().path());
    target_host.set_query(req.uri().query());

    let forwarded_req = client.request_from(target_host.as_str(), req.head());
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
