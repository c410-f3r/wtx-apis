use crate::test_data::json_placeholder::{GenericParams, JsonPlaceholder, PkgsAux};
use std::sync::LazyLock;
use wtx::{
  client_api_framework::{
    dnsn::SerdeJson,
    network::{transport::Transport, HttpParams},
  },
  http::{ClientTokioRustls, Method},
};

const DEFAULT_GP: GenericParams<'_> = GenericParams::new(None, Method::Get, None, &[]);
static CLIENT: LazyLock<ClientTokioRustls> =
  LazyLock::new(|| ClientTokioRustls::tokio_rustls(1).build());

create_http_test!(JsonPlaceholder, http(), albums, &*CLIENT, |pkgs_aux, trans| async {
  let _res = trans
    .send_recv_decode_contained(&mut pkgs_aux.albums().params(DEFAULT_GP).build(), pkgs_aux)
    .await
    .unwrap();
});

create_http_test!(JsonPlaceholder, http(), comments, &*CLIENT, |pkgs_aux, trans| async {
  let _res = trans
    .send_recv_decode_contained(&mut pkgs_aux.comments().params(DEFAULT_GP).build(), pkgs_aux)
    .await
    .unwrap();
});

create_http_test!(JsonPlaceholder, http(), photos, &*CLIENT, |pkgs_aux, trans| async {
  let _res = trans
    .send_recv_decode_contained(&mut pkgs_aux.photos().params(DEFAULT_GP).build(), pkgs_aux)
    .await
    .unwrap();
});

create_http_test!(JsonPlaceholder, http(), posts, &*CLIENT, |pkgs_aux, trans| async {
  let _res = trans
    .send_recv_decode_contained(&mut pkgs_aux.posts().params(DEFAULT_GP).build(), pkgs_aux)
    .await
    .unwrap();
});

create_http_test!(JsonPlaceholder, http(), todos, &*CLIENT, |pkgs_aux, trans| async {
  let _res = trans
    .send_recv_decode_contained(&mut pkgs_aux.todos().params(DEFAULT_GP).build(), pkgs_aux)
    .await
    .unwrap();
});

create_http_test!(JsonPlaceholder, http(), users, &*CLIENT, |pkgs_aux, trans| async {
  let _res = trans
    .send_recv_decode_contained(&mut pkgs_aux.users().params(DEFAULT_GP).build(), pkgs_aux)
    .await
    .unwrap();
});

fn http() -> (SerdeJson, HttpParams) {
  (SerdeJson, HttpParams::from_uri("https://jsonplaceholder.typicode.com:443"))
}
