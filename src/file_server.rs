use std::io::{Result as ioResult};

pub trait FileServer {
  fn serve(&self) -> ioResult<()>;
}
