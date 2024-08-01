use std::sync::LazyLock;
use tokio::runtime::Runtime;

pub(crate) static RUNTIME: LazyLock<Runtime> =
  LazyLock::new(|| tokio::runtime::Builder::new_current_thread().enable_all().build().unwrap());
