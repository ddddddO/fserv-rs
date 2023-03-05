/// ref: https://doc.rust-lang.org/std/net/struct.TcpListener.html
use std::net::{TcpListener, TcpStream};
use std::io::Result as ioResult;
use std::io::Write; // なぜこれがいるのか。TcpStreamは既にWrite実装されてるのではないのか
use std::io::Read; // これも

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

// ioResult<()> だと、エラー返してないよね？
fn handle_connection(mut stream: TcpStream) -> ioResult<()> {
    println!("stream: {:?}", stream);

    read_http(&mut stream)?;
    write_http(&mut stream)?;
    // stream.write(b"aaaa\nbbb")?;
    stream.flush()?;
    Ok(())
}

// TODO: streamの先頭行だけpeekしてプロトコルが何なのかチェックするfunc

fn read_http(stream: &mut TcpStream) -> ioResult<()> {
    let mut buf = [0;100]; // FIXME: 固定値やめたい
    stream.read(&mut buf)?;
    let s = std::str::from_utf8(&buf).unwrap(); // ここエラーハンドルしないと？
    println!("req:\n{}", s);
    Ok(())
}

fn write_http(stream: &mut TcpStream) -> ioResult<()> {
    stream.write(b"HTTP/1.1 200 OK\r\n")?;
    stream.write(b"Allow:: GET\r\n")?; // 可変借用2回以上okだっけ？
    stream.write(b"Connection: close\r\n")?;
    stream.write(b"\r\n")?;
    stream.write(b"\r\n")?;

    stream.write(b"Bodyyyyyyyyy\n")?;
    stream.write(b"Bodyyyyyyyyy\n")?;

    stream.write(b"\r\n")?;
    Ok(())
}