use std::net::{TcpListener, TcpStream};
use std::io::{Read, Result as ioResult};

use crate::file_server::{FileServer};

pub struct FtpServer {
  listener: TcpListener
}

impl FileServer for FtpServer {
  fn serve(&self) -> ioResult<()> {
    for stream in self.listener.incoming() {
      match stream {
          Ok(stream) => {
              match self.handle_connection(stream) {
                  Ok(..) => (),
                  Err(e) => println!("Handle error! {:?}", e),
              }
          }
          Err(e) => println!("Faild to connect? {:?}", e),
      }
    }
    Ok(())
  }
}

impl FtpServer {
  pub fn new(listener: TcpListener) -> Self {
    Self{
      listener: listener,
    }
  }

  // https://www.infraexpert.com/study/tcpip20.html
  fn handle_connection(&self, mut stream: TcpStream) -> ioResult<()> {
    // NOTE: clientから、ftp 127.0.0.1 を実行しても、サーバ側へは何も送ってなさそう。
    // TODO: なので、username/passwordの入力を求めるプロンプトを返して、受信するようにする
    // let mut buf = [0;1024];
    // stream.read(&mut buf)?;
    // if let Ok(s) = std::str::from_utf8(&buf) {
    //   println!("FTP receive:\n{}", s);
    // }
    Ok(())
  }
}