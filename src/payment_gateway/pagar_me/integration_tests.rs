use crate::payment_gateway::pagar_me::{PagarMe, PkgsAux};
use wtx::client_api_framework::{
  dnsn::SerdeJson,
  network::{transport::Transport, HttpParams},
};

create_http_test!(instance(), http(), balance, |pkgs_aux, trans| async {
  let _res = trans
    .send_retrieve_and_decode_contained(
      &mut pkgs_aux.recipient_balance().params("0").build(),
      pkgs_aux,
    )
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
