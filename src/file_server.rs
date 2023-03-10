use std::io::{Result as ioResult};

// TODO: trait側に、HttpServerで実装しているserveを持ってきてデフォルト実装とできないか
//       いや、共通にするのはまだ待った方がいいかも
pub trait FileServer {
  fn serve(&self) -> ioResult<()>;
}
