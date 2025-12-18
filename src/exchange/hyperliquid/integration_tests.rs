use crate::exchange::hyperliquid::{
  Hyperliquid, PkgsAux, WS_TESTNET_URI,
  order::{BulkOrder, OrderGrouping, OrderLimitParams, OrderReq, OrderTif, OrderTy},
};
use rust_decimal_macros::dec;
use std::sync::LazyLock;
use tokio::sync::Mutex;
use wtx::{
  client_api_framework::network::{WsParams, transport::SendingReceivingTransport},
  de::{HexDecMode, decode_hex_to_slice, format::SerdeJson},
};

static HYPERLIQUID: LazyLock<Mutex<Hyperliquid>> =
  LazyLock::new(|| Mutex::new(Hyperliquid::new(false)));

create_ws_test!(
  #[],
  WS_TESTNET_URI,
  &mut *HYPERLIQUID.lock().await,
  ws(),
  bulk_order,
  |pkgs_aux, trans| async {
    let mut signing_key = k256::ecdsa::SigningKey::from_bytes(_VARS.hyperliquid_sk.into()).unwrap();
    let _res = trans
      .send_pkg_recv_decode_contained(
        &mut pkgs_aux.bulk_order().data(
          BulkOrder {
            orders: &[
              OrderReq {
                asset: 1,
                is_buy: true,
                limit_price: dec!(1),
                size: dec!(1),
                reduce_only: false,
                ty: OrderTy::Limit(OrderLimitParams { tif: OrderTif::Gtc }),
                cloid: None
              }
            ],
            grouping: OrderGrouping::NormalTlsl,
            builder: None
          },
          &mut signing_key
        )
        .unwrap()
        .build(),
        pkgs_aux,
      )
      .await
      .unwrap()
      .data
      .result
      .unwrap()
      .response
      .payload
      .result
      .unwrap();
  }
);

create_ws_test!(
  #[],
  WS_TESTNET_URI,
  &mut *HYPERLIQUID.lock().await,
  ws(),
  order_status,
  |pkgs_aux, trans| async {
    let _res = trans
      .send_pkg_recv_decode_contained(
        &mut pkgs_aux.order_status().data(
          43413788610,
          "0xcb3b0727255755330c3e7e75CF08DdDdE4f6CF4C"
        )
        .unwrap()
        .build(),
        pkgs_aux,
      )
      .await
      .unwrap()
      .data
      .order
      .unwrap();
  }
);

fn ws() -> (SerdeJson, WsParams) {
  (SerdeJson, WsParams::default())
}
