use crate::payment_gateway::pagar_me::{PagarMe, PkgsAux};
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

create_http_test!(instance(), http(), balance, &*CLIENT, |pkgs_aux, trans| async {
  let _res = trans
    .send_recv_decode_contained(&mut pkgs_aux.recipient_balance().params("0").build(), pkgs_aux)
    .await
    .unwrap()
    .data
    .result
    .unwrap();
});

fn http() -> (SerdeJson, HttpParams) {
  (SerdeJson, HttpParams::from_uri("https://api.pagar.me/core/v5"))
}

fn instance() -> PagarMe {
  PagarMe::new(&std::env::var("PAGAR_ME_SK").unwrap()).unwrap()
}
