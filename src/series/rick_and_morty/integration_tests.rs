use crate::series::rick_and_morty::{PkgsAux, RickAndMorty};
use alloc::string::String;
use std::sync::LazyLock;
use wtx::{
  client_api_framework::network::{HttpParams, transport::SendingReceivingTransport},
  de::format::SerdeJson,
  http::client_pool::{ClientPoolBuilder, ClientPoolTokioRustls},
};

static CLIENT: LazyLock<ClientPoolTokioRustls<fn(&()), (), ()>> =
  LazyLock::new(|| ClientPoolBuilder::tokio_rustls(1).build());

create_http_test!(#[], RickAndMorty, http(), character, &*CLIENT, |pkgs_aux, trans| async {
  let _res = trans
    .send_pkg_recv_decode_contained(
      &mut pkgs_aux.character().data(&mut String::new(), 1).unwrap().build(),
      pkgs_aux,
    )
    .await
    .unwrap()
    .data
    .result
    .unwrap();
});

create_http_test!(#[ignore], RickAndMorty, http(), characters, &*CLIENT, |pkgs_aux, trans| async {
  let _res = trans
    .send_pkg_recv_decode_contained(
      &mut pkgs_aux.characters().data(&mut String::new(), "", "", 0, "", "", "").unwrap().build(),
      pkgs_aux,
    )
    .await
    .unwrap()
    .data
    .result
    .unwrap();
});

create_http_test!(#[ignore], RickAndMorty, http(), characters_by_ids, &*CLIENT, |pkgs_aux, trans| async {
  let _res = trans
    .send_pkg_recv_decode_contained(
      &mut pkgs_aux.characters_by_ids().data(&mut String::new(), &[1, 2]).unwrap().build(),
      pkgs_aux,
    )
    .await
    .unwrap()
    .data
    .result
    .unwrap();
});

create_http_test!(#[ignore], RickAndMorty, http(), episode, &*CLIENT, |pkgs_aux, trans| async {
  let _res = trans
    .send_pkg_recv_decode_contained(
      &mut pkgs_aux.episode().data(&mut String::new(), 1).unwrap().build(),
      pkgs_aux,
    )
    .await
    .unwrap()
    .data
    .result
    .unwrap();
});

create_http_test!(#[ignore], RickAndMorty, http(), episodes, &*CLIENT, |pkgs_aux, trans| async {
  let _res = trans
    .send_pkg_recv_decode_contained(
      &mut pkgs_aux.episodes().data(&mut String::new(), "", "", 0).unwrap().build(),
      pkgs_aux,
    )
    .await
    .unwrap()
    .data
    .result
    .unwrap();
});

create_http_test!(#[ignore], RickAndMorty, http(), episodes_by_ids, &*CLIENT, |pkgs_aux, trans| async {
  let _res = trans
    .send_pkg_recv_decode_contained(
      &mut pkgs_aux.episodes_by_ids().data(&mut String::new(), &[1, 2]).unwrap().build(),
      pkgs_aux,
    )
    .await
    .unwrap()
    .data
    .result
    .unwrap();
});

create_http_test!(#[ignore], RickAndMorty, http(), location, &*CLIENT, |pkgs_aux, trans| async {
  let _res = trans
    .send_pkg_recv_decode_contained(
      &mut pkgs_aux.location().data(&mut String::new(), 1).unwrap().build(),
      pkgs_aux,
    )
    .await
    .unwrap()
    .data
    .result
    .unwrap();
});

create_http_test!(#[ignore], RickAndMorty, http(), locations, &*CLIENT, |pkgs_aux, trans| async {
  let _res = trans
    .send_pkg_recv_decode_contained(
      &mut pkgs_aux.locations().data(&mut String::new(), "", "", 0, "").unwrap().build(),
      pkgs_aux,
    )
    .await
    .unwrap()
    .data
    .result
    .unwrap();
});

create_http_test!(#[ignore], RickAndMorty, http(), locations_by_ids, &*CLIENT, |pkgs_aux, trans| async {
  let _res = trans
    .send_pkg_recv_decode_contained(
      &mut pkgs_aux.locations_by_ids().data(&mut String::new(), &[1, 2]).unwrap().build(),
      pkgs_aux,
    )
    .await
    .unwrap()
    .data
    .result
    .unwrap();
});

fn http() -> (SerdeJson, HttpParams) {
  (SerdeJson, HttpParams::from_uri("https://rickandmortyapi.com/graphql".into()))
}
