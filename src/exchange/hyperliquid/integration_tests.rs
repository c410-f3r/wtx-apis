use crate::exchange::hyperliquid::{PkgsAux, Hyperliquid};
use std::sync::LazyLock;
use tokio::sync::Mutex;
use wtx::{
  client_api_framework::network::{HttpParams, transport::SendingReceivingTransport},
  de::format::SerdeJson,
  http::client_pool::{ClientPoolBuilder, ClientPoolTokioRustls},
};

static CLIENT: LazyLock<ClientPoolTokioRustls<fn(&()), (), ()>> =
  LazyLock::new(|| ClientPoolBuilder::tokio_rustls(1).build());
static HYPERLIQUID: LazyLock<Mutex<Hyperliquid>> = LazyLock::new(|| Mutex::new(Hyperliquid::new()));

create_http_test!(
  #[],
  "",
  http(),
  get_balance,
  &*CLIENT,
  |pkgs_aux, trans| async {
    let _res = trans
      .send_pkg_recv_decode_contained(
        &mut pkgs_aux.place_order().build(),
        pkgs_aux,
      )
      .await
      .unwrap()
      .result
      .unwrap();
  }
);

fn http() -> (SerdeJson, HttpParams) {
  (SerdeJson, HttpParams::from_uri("".into()))
}
