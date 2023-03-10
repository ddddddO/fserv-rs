use std::net::{TcpListener};
use std::io::{Result as ioResult};

use fserv_rs::http::{HttpServer};
use fserv_rs::ftp::{FtpServer};
use fserv_rs::file_server::{FileServer};

fn main() -> ioResult<()> {
    println!("Launch fserv-rs.");

    // HTTP Server
    // let listener = TcpListener::bind("127.0.0.1:8081")?;
    // let server = HttpServer::new(listener);

    // FTP Server
    const CONTROL_CONNECTION_PORT: &str = "21";
    let listener = TcpListener::bind(format!("127.0.0.1:{}", CONTROL_CONNECTION_PORT))?;
    let server = FtpServer::new(listener);

    serve(server)
}

// 静的ディスパッチ
fn serve(server: impl FileServer) -> ioResult<()> {
    server.serve()
}