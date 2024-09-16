use crate::blockchain::ethereum::{BlockNumber, Ethereum, PkgsAux};
use core::time::Duration;
use std::sync::LazyLock;
use tokio::sync::Mutex;
use wtx::{
  client_api_framework::{
    misc::{RequestLimit, RequestThrottling},
    network::{transport::Transport, HttpParams},
  },
  data_transformation::dnsn::SerdeJson,
  http::ClientFrameworkTokioRustls,
};

static CLIENT: LazyLock<ClientFrameworkTokioRustls> =
  LazyLock::new(|| ClientFrameworkTokioRustls::tokio_rustls(1).build());
static ETHEREUM: LazyLock<Mutex<Ethereum>> = LazyLock::new(|| {
  Mutex::new(Ethereum::new(Some(RequestThrottling::from_rl(RequestLimit::new(
    1,
    Duration::from_secs(1),
  )))))
});

create_http_test!(
  &mut *ETHEREUM.lock().await,
  http(),
  eth_block_number,
  &*CLIENT,
  |pkgs_aux, trans| async {
    let _res = trans
      .send_recv_decode_contained(&mut pkgs_aux.eth_block_number().build(), pkgs_aux)
      .await
      .unwrap()
      .result
      .unwrap();
  }
);

create_http_test!(
  &mut *ETHEREUM.lock().await,
  http(),
  eth_block_transaction_count_by_number,
  &*CLIENT,
  |pkgs_aux, trans| async {
    let _res = trans
      .send_recv_decode_contained(
        &mut pkgs_aux
          .eth_block_transaction_count_by_number()
          .data([&BlockNumber::Number(15228994)])
          .build(),
        pkgs_aux,
      )
      .await
      .unwrap()
      .result
      .unwrap();
  }
);

create_http_test!(
  &mut *ETHEREUM.lock().await,
  http(),
  eth_get_balance,
  &*CLIENT,
  |pkgs_aux, trans| async {
    let _res = trans
      .send_recv_decode_contained(
        &mut pkgs_aux
          .eth_get_balance()
          .data("0xd6216fc19db775df9774a6e33526131da7d19a2c", &BlockNumber::Latest)
          .build(),
        pkgs_aux,
      )
      .await
      .unwrap()
      .result
      .unwrap();
  }
);

fn http() -> (SerdeJson, HttpParams) {
  (SerdeJson, HttpParams::from_uri("https://eth-mainnet.public.blastapi.io"))
}
