use std::net::{TcpListener};
use std::io::{Result as ioResult};

use fserv_rs::http::{HttpServer};

fn main() -> ioResult<()> {
    println!("Launch fserv-rs.");

    let listener = TcpListener::bind("127.0.0.1:8081")?;
    let server = HttpServer::new(listener);
    server.serve()?;

    Ok(())
}
