use std::net::{TcpListener};
use std::io::{Result as ioResult};

use fserv_rs::http::{HttpServer};
use fserv_rs::file_server::{FileServer};

fn main() -> ioResult<()> {
    println!("Launch fserv-rs.");

    let listener = TcpListener::bind("127.0.0.1:8081")?;
    let server = HttpServer::new(listener);
    serve(&server)
}

// TODO: let server: FileServer = HttpServer::new(listener); みたいに、短く書けないか？
fn serve<T: FileServer>(server: &T) -> ioResult<()> {
    server.serve()
}