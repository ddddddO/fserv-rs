use std::io::{Result as ioResult};

// TODO: trait側に、HttpServerで実装しているserveを持ってきてデフォルト実装とできないか
pub trait FileServer {
  fn serve(&self) -> ioResult<()>;
}
