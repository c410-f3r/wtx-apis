use crate::calendar::nager_date::{NagerDate, PkgsAux};
use std::sync::LazyLock;
use wtx::{
  client_api_framework::network::{transport::Transport, HttpParams},
  data_transformation::dnsn::SerdeJson,
  http::ClientFrameworkTokioRustls,
};

static CLIENT: LazyLock<ClientFrameworkTokioRustls> =
  LazyLock::new(|| ClientFrameworkTokioRustls::tokio_rustls(1).build());

create_http_test!(NagerDate, http(), v3_available_countries, &*CLIENT, |pkgs_aux, trans| async {
  let _res = trans
    .send_recv_decode_contained(&mut pkgs_aux.v3_available_countries().build(), pkgs_aux)
    .await
    .unwrap();
});

create_http_test!(NagerDate, http(), v3_country_info, &*CLIENT, |pkgs_aux, trans| async {
  let _res = trans
    .send_recv_decode_contained(&mut pkgs_aux.v3_country_info().params("es").build(), pkgs_aux)
    .await
    .unwrap();
});

create_http_test!(NagerDate, http(), v3_long_weekend, &*CLIENT, |pkgs_aux, trans| async {
  let _res = trans
    .send_recv_decode_contained(
      &mut pkgs_aux.v3_long_weekend().params(2020, "es").build(),
      pkgs_aux,
    )
    .await
    .unwrap();
});

create_http_test!(
  NagerDate,
  http(),
  v3_next_public_holidays_worldwide,
  &*CLIENT,
  |pkgs_aux, trans| async {
    let _res = trans
      .send_recv_decode_contained(
        &mut pkgs_aux.v3_next_public_holidays_worldwide().build(),
        pkgs_aux,
      )
      .await
      .unwrap();
  }
);

create_http_test!(NagerDate, http(), v3_next_public_holidays, &*CLIENT, |pkgs_aux, trans| async {
  let _res = trans
    .send_recv_decode_contained(
      &mut pkgs_aux.v3_next_public_holidays().params("es").build(),
      pkgs_aux,
    )
    .await
    .unwrap();
});

create_http_test!(NagerDate, http(), v3_public_holidays, &*CLIENT, |pkgs_aux, trans| async {
  let _res = trans
    .send_recv_decode_contained(
      &mut pkgs_aux.v3_public_holidays().params(2000, "es").build(),
      pkgs_aux,
    )
    .await
    .unwrap();
});

fn http() -> (SerdeJson, HttpParams) {
  (SerdeJson, HttpParams::from_uri("https://date.nager.at:443"))
}
