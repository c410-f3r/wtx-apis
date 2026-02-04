use crate::{
  exchange::aster::{
    Aster, OrderPostReqParams, OrderReqParams, OrderSide, OrderType, PkgsAux,
    TESTNET_SPOT_HTTP_URI, TESTNET_SPOT_WS_URI, UserTradesReqParams,
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
  misc::{Secret, SensitiveBytes},
  rng::{ChaCha20, SeedableRng},
};

static ASTER: LazyLock<Mutex<Aster>> = LazyLock::new(|| {
  let mut secret = [0; 32];
  let _ = wtx::de::decode_hex(_VARS.aster_secret.as_bytes(), &mut secret).unwrap();
  Mutex::new(Aster::new(
    true,
    RequestCounter::new(RequestLimit::unlimited()),
    Secret::new(
      SensitiveBytes::new_locked(secret.as_mut_slice()).unwrap(),
      &mut ChaCha20::from_std_random().unwrap(),
    )
    .unwrap(),
    _VARS.aster_signer.clone(),
    _VARS.aster_user.clone(),
  ))
});
static CLIENT: LazyLock<ClientPoolTokioRustls<fn(&()), ()>> =
  LazyLock::new(|| ClientPoolBuilder::tokio_rustls(1).build());

create_http_test!(
  #[],
  &mut *ASTER.lock().await,
  http(),
  account,
  &*CLIENT,
  |pkgs_aux, trans| async {
    let pkg = &mut pkgs_aux
      .account()
      .data(None)
      .unwrap()
      .build();
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
  exchange_info,
  &*CLIENT,
  |pkgs_aux, trans| async {
    let pkg = &mut pkgs_aux.exchange_info().build();
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
  listen_key,
  &*CLIENT,
  |pkgs_aux, trans| async {
    let pkg = &mut pkgs_aux.listen_key().data(None).unwrap().build();
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
  order_get,
  &*CLIENT,
  |pkgs_aux, trans| async {
    let pkg = &mut pkgs_aux
      .order_get()
      .data(&OrderReqParams {
        order_id: Some(262221260),
        orig_client_order_id: None,
        sign_params: None,
        symbol: "ASTERUSDT".try_into().unwrap(),
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
  order_post,
  &*CLIENT,
  |pkgs_aux, trans| async {
    let mut fun = async |side| {
      let pkg = &mut pkgs_aux
        .order_post()
        .data(&OrderPostReqParams {
          new_client_order_id: None,
          price: None,
          quantity: Some(Decimal::from_parts(30, 0, 0, false, 0)),
          quote_order_qty: None,
          side,
          cex_sign_params: None,
          stop_price: None,
          symbol: "ASTERUSDT".try_into().unwrap(),
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
    };
    fun(OrderSide::Buy).await;
    fun(OrderSide::Sell).await;
  }
);

create_http_test!(
  #[],
  &mut *ASTER.lock().await,
  http(),
  user_trades,
  &*CLIENT,
  |pkgs_aux, trans| async {
    let pkg = &mut pkgs_aux
      .user_trades()
      .data(&UserTradesReqParams {
        symbol: None,
        order_id: Some(262221260),
        start_time: None,
        end_time: None,
        from_id: None,
        limit: None,
        cex_sign_params: None
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
  TESTNET_SPOT_WS_URI,
  &mut *ASTER.lock().await,
  ws(),
  book_ticker,
  |pkgs_aux, trans| async {
    let pkg = &mut pkgs_aux.ws().data(1, "SUBSCRIBE", ["btcusdt@bookTicker"]).build();
    pkgs_aux.log_body();
    [trans
      .send_pkg_recv_decode_contained(pkg, pkgs_aux)
      .await
      .unwrap()]
  }
);

fn http() -> (SerdeJson, HttpParams) {
  (SerdeJson, HttpParams::from_uri(TESTNET_SPOT_HTTP_URI.into()))
}

fn ws() -> (SerdeJson, WsParams) {
  (SerdeJson, WsParams::default())
}
