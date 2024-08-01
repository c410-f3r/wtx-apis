use crate::blockchain::ethereum::{BlockNumber, Ethereum, PkgsAux};
use std::sync::LazyLock;
use wtx::{
  client_api_framework::{
    dnsn::SerdeJson,
    network::{transport::Transport, HttpParams},
  },
  http::ClientTokioRustls,
};

static CLIENT: LazyLock<ClientTokioRustls> =
  LazyLock::new(|| ClientTokioRustls::tokio_rustls(1).build());

create_http_test!(Ethereum, http(), eth_block_number, &*CLIENT, |pkgs_aux, trans| async {
  let _res = trans
    .send_recv_decode_contained(&mut pkgs_aux.eth_block_number().build(), pkgs_aux)
    .await
    .unwrap()
    .result
    .unwrap();
});

create_http_test!(
  Ethereum,
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

create_http_test!(Ethereum, http(), eth_get_balance, &*CLIENT, |pkgs_aux, trans| async {
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
});

fn http() -> (SerdeJson, HttpParams) {
  (SerdeJson, HttpParams::from_uri("https://cloudflare-eth.com:443"))
}
