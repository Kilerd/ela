#![feature(async_await)]

use std::future::Future;
use runtime::net::{TcpListener, TcpStream};
use futures::prelude::*;
use futures::try_join;

#[runtime::main]
async fn main() -> std::io::Result<()> {
    let mut listener = TcpListener::bind("127.0.0.1:8081")?;
    println!("Listening on {}", listener.local_addr()?);

    listener
        .incoming()
        .try_for_each_concurrent(None, async move |client|{
        runtime::spawn(async move {
            let server = TcpStream::connect("127.0.0.1:8080").await?;
            println!("proxy {} to {}", client.peer_addr()?, server.peer_addr()?);
            let (cr, cw) = &mut client.split();
            let (sr, sw) = &mut server.split();
            let a = cr.copy_into(sw);
            let b = sr.copy_into(cw);
            try_join!(a, b);
            Ok::<(), std::io::Error>(())
        })
            .await
    })
        .await?;

    Ok(())
}