use std::fs;
use std::fs::File;
use std::path::Path;
use std::net::{TcpListener, TcpStream}; /// ref: https://doc.rust-lang.org/std/net/struct.TcpListener.html
use std::io::Result as ioResult;
use std::io::Write; // なぜこれがいるのか。TcpStreamは既にWrite実装されてるのではないのか
use std::io::Read; // これも

fn main() -> ioResult<()> {
    println!("Launch fserv-rs.");

    let listener = TcpListener::bind("127.0.0.1:8081")?;
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                match handle_connection(stream) {
                    Ok(..) => (),
                    Err(e) => println!("Handle error! {:?}", e),
                }
            }
            Err(e) => println!("Faild to connect? {:?}", e),
        }
    }
    Ok(())
}

fn handle_connection(mut stream: TcpStream) -> ioResult<()> {
    let path = read_http(&mut stream)?;
    if !valid_path(&path) {
        write_http_notfound(&mut stream)?;
        return Ok(());
    }

    let contents = get_file_contents(&path)?;
    write_http(&mut stream, &contents)?;
    Ok(())
}

// TODO: streamの先頭行だけpeekしてプロトコルが何なのかチェックするfunc

fn read_http(stream: &mut TcpStream) -> ioResult<String> {
    let mut buf = [0;100]; // FIXME: 固定値やめたい
    stream.read(&mut buf)?;

    // TODO: Errの場合err返す。が、↑で返すErrの型が違う場合どうすれば？
    if let Ok(s) = std::str::from_utf8(&buf) {
        let lines = s.lines().collect::<Vec<&str>>();
        if let Some((_, path, _)) = parse_request_line(lines[0]) {
            let copied_path = String::from(path);
            return Ok(copied_path);
        }
    }

    Ok("".to_string())
}

fn parse_request_line(line: &str) -> Option<(&str, &str, &str)> {
    let parsed: Vec<&str> = line.split(" ").collect();
    if parsed.len() != 3 {
        return None; // ここNoneなのかな。httpのステータスラインでないものが来てる、ということが確実ならErrかなとも思う。
    }

    let (method, path, protocol) = (parsed[0], parsed[1], parsed[2]);
    println!("Method: {}", method);
    println!("Path: {}", path);
    println!("Protocol: {}", protocol);

    Some((method, path, protocol))
}

// https://doc.rust-jp.rs/book-ja/ch12-02-reading-a-file.html
// TODO: バリデーションとか色々
fn get_file_contents(path: &str) -> ioResult<String> {
    let mut name = path.to_string();
    name.remove(0); // 先頭の"/"削除
    let file_path = Path::new(".").join(name);
    if file_path.is_dir() {
        if let Some(dir) = file_path.to_str() {
            let page = generate_index_page(dir);
            return Ok(page);
        };
    };

    let mut f = File::open(file_path)?;
    let mut contents = String::new();
    f.read_to_string(&mut contents)?;
    Ok(contents)
}

// TODO: この実装をやめ、ちゃんと今いるパスのls一覧に含まれてるかそうでないかチェック
fn valid_path(path: &str) -> bool {
    match path {
        "" => false,
        "/favicon.ico" => false,
        "/service_worker.js" => false,
        _ => true,
    }
}

fn generate_index_page(path: &str) -> String {
    let mut buf = String::from("<html><head></head><body><pre>\n");

    if let Ok(entries) = fs::read_dir(path) {
        for entry in entries {
            let e_ref = entry.as_ref();
            let link = Path::new(path);

            if let Some(p) = e_ref.unwrap().file_name().to_str() {
                buf.push_str("<a href=\"");
                if let Some(l) = link.join(p).to_str() {
                    let mut parsed: Vec<&str> = l.split("/").collect();
                    if parsed.len() >= 4 {
                        buf.push_str(".");
                        // hrefで欲しいのは、(parsed.len()-2)..の範囲のパス
                        for p in parsed.drain((parsed.len()-2)..) {
                            buf.push_str(format!("/{}", p).as_str());
                        }
                    } else {
                        buf.push_str(l);
                    }
                }

                buf.push_str("\">");
                buf.push_str(p);
                if e_ref.expect("not found path").path().is_dir() {
                    buf.push_str("/");
                }
                buf.push_str("</a>\n");
            }
        }
    }

    buf + "</pre></body></html>"
}

fn write_http(stream: &mut TcpStream, contents: &str) -> ioResult<()> {
    stream.write(b"HTTP/1.1 200 OK\r\n")?;
    stream.write(format!("Content-Length: {}\r\n", contents.len()).as_bytes())?;
    // TODO: レスポンスによって、ContentType変えたい
    // stream.write(b"Content-Type: text/rust; charset=utf-8\r\n")?;
    stream.write(b"Connection: close\r\n")?;
    stream.write(b"\r\n")?;

    stream.write(contents.as_bytes())?;

    stream.write(b"\r\n")?;
    stream.write(b"\r\n")?;

    stream.flush()?;
    Ok(())
}

fn write_http_notfound(stream: &mut TcpStream) -> ioResult<()> {
    stream.write(b"HTTP/1.1 404 Not Found\r\n")?;
    stream.write(b"Connection: close\r\n")?;
    stream.write(b"\r\n")?;

    stream.write(b"404 Not Found")?;

    stream.write(b"\r\n")?;
    stream.write(b"\r\n")?;

    stream.flush()?;
    Ok(())
}
