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

    let path = read_http(&mut stream)?;
    println!("PAth!: {}", path);

    write_http(&mut stream)?;
    stream.flush()?;
    Ok(())
}

// TODO: streamの先頭行だけpeekしてプロトコルが何なのかチェックするfunc

fn read_http(stream: &mut TcpStream) -> ioResult<String> {
    let mut buf = [0;100]; // FIXME: 固定値やめたい
    stream.read(&mut buf)?;

    let s = std::str::from_utf8(&buf).unwrap(); // ここエラーハンドルしないと？
    println!("Request:\n{}", s);

    let lines = s.lines().collect::<Vec<&str>>();
    let (_, path, _) = parse_request_line(lines[0]);

    let copied_path = String::from(path);
    Ok(copied_path)
}

fn parse_request_line(line: &str) -> (&str, &str, &str) {
    let parsed: Vec<&str> = line.split(" ").collect();
    let (method, path, protocol) = (parsed[0], parsed[1], parsed[2]); // FIXME: lenチェック
    println!("Method: {}", method);
    println!("Path: {}", path);
    println!("Protocol: {}", protocol);

    (method, path, protocol)
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