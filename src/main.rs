/// ref: https://doc.rust-lang.org/std/net/struct.TcpListener.html
use std::net::{TcpListener, TcpStream};
use std::io::Result as ioResult;
use std::io::Write; // なぜこれがいるのか。tcpStreamは既にWrite実装されてるのではないのか

fn main() -> ioResult<()> {
    println!("Lauch fserv-rs.");

    let listener = TcpListener::bind("127.0.0.1:8080")?;
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                handle_connection(stream)?;
            }
            Err(e) => println!("Error! {:?}", e),
        }
    }
    Ok(())
}

fn handle_connection(mut stream: TcpStream) -> ioResult<()> {
    println!("stream: {:?}", stream);
    stream.write(b"aaaa\nbbb")?;
    stream.flush()?;
    Ok(())
}

// fn write_http(mut stream: TcpStream) {

// }