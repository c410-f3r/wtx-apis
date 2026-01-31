use crate::{
  exchange::aster::{
    Aster, OrderSide, OrderType, PROD_SPOT_HTTP_URI, PROD_SPOT_WS_URI, PkgsAux,
    V1OrderGetReqParams, V1OrderPostReqParams, sign_params::SignParams,
  },
  tests::_VARS,
};
use rust_decimal::Decimal;
use std::sync::LazyLock;
use tokio::sync::Mutex;
use wtx::{
  client_api_framework::{
    misc::{RequestCounter, RequestLimit},
    network::{HttpParams, WsParams, transport::SendingReceivingTransport},
  },
  de::format::SerdeJson,
  http::client_pool::{ClientPoolBuilder, ClientPoolTokioRustls},
  misc::{Secret, SensitiveBytes, timestamp_millis_str},
  rng::{ChaCha20, SeedableRng},
};

static ASTER: LazyLock<Mutex<Aster>> = LazyLock::new(|| {
  let mut secret_key = _VARS.aster_secret_key.clone().into_bytes();
  Mutex::new(Aster::new(
    _VARS.aster_api_key.clone(),
    RequestCounter::new(RequestLimit::unlimited()),
    Secret::new(
      SensitiveBytes::new_locked(secret_key.as_mut_slice()).unwrap(),
      &mut ChaCha20::from_std_random().unwrap(),
    )
    .unwrap(),
  ))
});
static CLIENT: LazyLock<ClientPoolTokioRustls<fn(&()), ()>> =
  LazyLock::new(|| ClientPoolBuilder::tokio_rustls(1).build());

create_http_test!(
  #[],
  &mut *ASTER.lock().await,
  http(),
  v1_order_get,
  &*CLIENT,
  |pkgs_aux, trans| async {
    let (now, _) = timestamp_millis_str().unwrap();
    let pkg = &mut pkgs_aux
      .v1_order_get()
      .data(&V1OrderGetReqParams {
        order_id: Some(290307),
        orig_client_order_id: None,
        sign_params: SignParams { timestamp: now, recv_window: None },
        symbol: "USDCUSDT".try_into().unwrap(),
      })
      .unwrap()
      .build();
    pkgs_aux.log_body();
    let _rslt = trans
      .send_pkg_recv_decode_contained(pkg, pkgs_aux)
      .await
      .unwrap()
      .data;
  }
);

create_http_test!(
  #[],
  &mut *ASTER.lock().await,
  http(),
  v1_order_post,
  &*CLIENT,
  |pkgs_aux, trans| async {
    let (now, _) = timestamp_millis_str().unwrap();
    let pkg = &mut pkgs_aux
      .v1_order_post()
      .data(&V1OrderPostReqParams {
        new_client_order_id: None,
        price: None,
        quantity: Some(Decimal::from_parts(5, 0, 0, false, 0)),
        quote_order_qty: None,
        side: OrderSide::Sell,
        sign_params: SignParams { timestamp: now, recv_window: None },
        stop_price: None,
        symbol: "USDCUSDT".try_into().unwrap(),
        time_in_force: None,
        ty: OrderType::Market,
      })
      .unwrap()
      .build();
    let _rslt = trans
      .send_pkg_recv_decode_contained(pkg, pkgs_aux)
      .await
      .unwrap()
      .data;
  }
);

create_ws_test!(
  #[],
  PROD_SPOT_WS_URI,
  &mut *ASTER.lock().await,
  ws(),
  book_ticker,
  |pkgs_aux, trans| async {
    [trans
      .send_pkg_recv_decode_contained(&mut pkgs_aux.subscribe().data(["btcusdt@bookTicker"]).build(), pkgs_aux)
      .await
      .unwrap()
      .result
      .unwrap()]
  }
);

fn http() -> (SerdeJson, HttpParams) {
  (SerdeJson, HttpParams::from_uri(PROD_SPOT_HTTP_URI.into()))
}

fn ws() -> (SerdeJson, WsParams) {
  (SerdeJson, WsParams::default())
}
