use crate::test_data::json_placeholder::{GenericParams, JsonPlaceholder, PkgsAux};
use wtx::{
  client_api_framework::{
    dnsn::SerdeJson,
    network::{transport::Transport, HttpParams},
  },
  http::Method,
};

const DEFAULT_GP: GenericParams<'_> = GenericParams::new(None, Method::Get, None, &[]);

create_http_test!(JsonPlaceholder, http(), albums, |pkgs_aux, trans| async {
  let _res = trans
    .send_retrieve_and_decode_contained(&mut pkgs_aux.albums().params(DEFAULT_GP).build(), pkgs_aux)
    .await
    .unwrap();
});

create_http_test!(JsonPlaceholder, http(), comments, |pkgs_aux, trans| async {
  let _res = trans
    .send_retrieve_and_decode_contained(
      &mut pkgs_aux.comments().params(DEFAULT_GP).build(),
      pkgs_aux,
    )
    .await
    .unwrap();
});

create_http_test!(JsonPlaceholder, http(), photos, |pkgs_aux, trans| async {
  let _res = trans
    .send_retrieve_and_decode_contained(&mut pkgs_aux.photos().params(DEFAULT_GP).build(), pkgs_aux)
    .await
    .unwrap();
});

create_http_test!(JsonPlaceholder, http(), posts, |pkgs_aux, trans| async {
  let _res = trans
    .send_retrieve_and_decode_contained(&mut pkgs_aux.posts().params(DEFAULT_GP).build(), pkgs_aux)
    .await
    .unwrap();
});

create_http_test!(JsonPlaceholder, http(), todos, |pkgs_aux, trans| async {
  let _res = trans
    .send_retrieve_and_decode_contained(&mut pkgs_aux.todos().params(DEFAULT_GP).build(), pkgs_aux)
    .await
    .unwrap();
});

create_http_test!(JsonPlaceholder, http(), users, |pkgs_aux, trans| async {
  let _res = trans
    .send_retrieve_and_decode_contained(&mut pkgs_aux.users().params(DEFAULT_GP).build(), pkgs_aux)
    .await
    .unwrap();
});

fn http() -> (SerdeJson, HttpParams) {
  (SerdeJson, HttpParams::from_uri("https://jsonplaceholder.typicode.com"))
}
