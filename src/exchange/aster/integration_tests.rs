use crate::{
  exchange::aster::{
    Aster, DepositAssetsReqParams, DepthReqParams, MarketReqParams, OrderPostReqParams,
    OrderReqParams, OrderSide, OrderType, PkgsAux, TESTNET_CHAIN_ID, TESTNET_SPOT_HTTP_URI,
    TESTNET_SPOT_WS_URI, UserTradesReqParams, WalletTransferReqParams, WalletTransferTy,
  },
  tests::_VARS,
};
use rust_decimal::Decimal;
use std::sync::LazyLock;
use tokio::sync::Mutex;
use wtx::{
  calendar::timestamp_str,
  client_api_framework::{
    misc::{RequestCounter, RequestLimit},
    network::{HttpParams, WsParams, transport::SendingReceivingTransport},
  },
  codec::format::SerdeJson,
  http::client_pool::{ClientPoolBuilder, ClientPoolTokioRustls},
  misc::{Secret, SecretContext},
  rng::{ChaCha20, CryptoSeedableRng},
};

static ASTER: LazyLock<Mutex<Aster>> = LazyLock::new(|| {
  let mut secret = [0; 32];
  let _ = wtx::codec::decode_hex(_VARS.aster_secret.as_bytes(), &mut secret).unwrap();
  let mut rng = ChaCha20::from_std_random().unwrap();
  let secret_context = SecretContext::new(&mut rng).unwrap();
  Mutex::new(Aster::new(
    TESTNET_CHAIN_ID,
    true,
    RequestCounter::new(RequestLimit::unlimited()),
    Secret::new(secret.as_mut_slice(), &mut rng, secret_context).unwrap(),
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
  commission_rate,
  &*CLIENT,
  |pkgs_aux, trans| async {
    let pkg = &mut pkgs_aux
      .commission_rate()
      .data(&MarketReqParams {
          symbol: "ASTERUSDT",
          sign_params: None
        }).unwrap()
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
  deposit_assets,
  &*CLIENT,
  |pkgs_aux, trans| async {
    let local_pkgs_aux = &mut **pkgs_aux;
    let _res = crate::exchange::aster::deposit_assets(
      &DepositAssetsReqParams { chain_ids: "56", networks: None, account_type: "spot" },
      (&mut local_pkgs_aux.api, &mut local_pkgs_aux.drsr, &mut *trans, &mut local_pkgs_aux.tp)
    )
    .await
    .unwrap();
  }
);

create_http_test!(
  #[],
  &mut *ASTER.lock().await,
  http(),
  depth,
  &*CLIENT,
  |pkgs_aux, trans| async {
    let pkg = &mut pkgs_aux.depth().data(&DepthReqParams {
      symbol: "ASTERUSDT",
      limit: None
    }).unwrap().build();
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
  open_orders,
  &*CLIENT,
  |pkgs_aux, trans| async {
    let pkg = &mut pkgs_aux.open_orders().data(&MarketReqParams {
      symbol: "ASTERUSDT",
      sign_params: None
    }).unwrap().build();
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
        symbol: "ASTERUSDT",
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
          symbol: "ASTERUSDT",
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

create_http_test!(
  #[],
  &mut *ASTER.lock().await,
  http(),
  wallet_transfer,
  &*CLIENT,
  |pkgs_aux, trans| async {
    let mut fun = async |kind_type| {
      let pkg = &mut pkgs_aux
        .wallet_transfer()
        .data(&WalletTransferReqParams {
          amount: Decimal::ONE,
          asset: "ASTER",
          client_tran_id: timestamp_str(|el| el.as_nanos()).unwrap().1.as_str(),
          kind_type
        })
        .unwrap()
        .build();
      let _rslt = trans
        .send_pkg_recv_decode_contained(pkg, pkgs_aux)
        .await
        .unwrap()
        .data;
    };
    fun(WalletTransferTy::FutureSpot).await;
    fun(WalletTransferTy::SpotFuture).await;
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
