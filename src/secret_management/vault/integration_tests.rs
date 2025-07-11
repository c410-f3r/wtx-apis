use crate::secret_management::vault::{PkgsAux, Vault};
use alloc::string::String;
use std::sync::LazyLock;
use tokio::sync::Mutex;
use wtx::{
  client_api_framework::network::{HttpParams, transport::SendingReceivingTransport},
  de::format::SerdeJson,
  http::client_pool::{ClientPoolBuilder, ClientPoolTokioRustls},
};

static CLIENT: LazyLock<ClientPoolTokioRustls<fn(&()), (), ()>> =
  LazyLock::new(|| ClientPoolBuilder::tokio_rustls(1).build());
static URI: LazyLock<String> = LazyLock::new(|| std::env::var("VAULT_URI").unwrap());
static VAULT: LazyLock<Mutex<Vault>> = LazyLock::new(|| {
  let token = std::env::var("VAULT_TOKEN").unwrap();
  Mutex::new(Vault::new(token))
});

create_http_test!(
  #[ignore],
  &mut *VAULT.lock().await,
  http(),
  kv2_read_secret_version,
  &*CLIENT,
  |pkgs_aux, trans| async {
    let _res = trans
      .send_pkg_recv_decode_contained(
        &mut pkgs_aux.kv2_read_secret_version().params("test", "test", None).build(),
        pkgs_aux,
      )
      .await;
  }
);

fn http() -> (SerdeJson, HttpParams) {
  (SerdeJson, HttpParams::from_uri(URI.clone()))
}
