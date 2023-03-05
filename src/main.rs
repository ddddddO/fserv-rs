use std::fs::File;
use std::net::{TcpListener, TcpStream}; /// ref: https://doc.rust-lang.org/std/net/struct.TcpListener.html
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
    let contents = get_file_contents(&path)?;
    println!("contents: {}", contents);

    write_http(&mut stream, &contents)?;
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

// https://doc.rust-jp.rs/book-ja/ch12-02-reading-a-file.html
// TODO: バリデーションとか色々
fn get_file_contents(path: &str) -> ioResult<String> {
    let file_path = ".".to_owned() + path;

    let mut f = File::open(file_path).expect("file not found");
    let mut contents = String::new();
    f.read_to_string(&mut contents)?;
    Ok(contents)
}

fn write_http(stream: &mut TcpStream, contents: &str) -> ioResult<()> {
    stream.write(b"HTTP/1.1 200 OK\r\n")?;
    stream.write(b"Allow:: GET\r\n")?; // 可変借用2回以上okだっけ？
    stream.write(b"Connection: close\r\n")?;
    stream.write(b"\r\n")?;
    stream.write(b"\r\n")?;

    stream.write(contents.as_bytes())?;

    stream.write(b"\r\n")?;
    stream.write(b"\r\n")?;
    Ok(())
}