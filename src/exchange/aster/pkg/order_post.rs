use crate::{
  PairName,
  exchange::aster::{
    CexSignParams, ClientOrderIdTy, OrderSide, OrderStatus, OrderType, TimeInForce,
  },
};
use rust_decimal::Decimal;

/// Structure sent when creating orders
#[derive(Debug, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct OrderPostReqParams {
  /// Trading pair symbol.
  pub symbol: PairName,
  /// See [`OrderType`].
  #[serde(rename = "type")]
  pub ty: OrderType,
  /// Order side (buy or sell).
  pub side: OrderSide,
  /// See [`TimeInForce`].
  #[serde(skip_serializing_if = "Option::is_none")]
  pub time_in_force: Option<TimeInForce>,
  /// Order quantity
  #[serde(skip_serializing_if = "Option::is_none")]
  pub quantity: Option<Decimal>,
  /// Quote quantity
  #[serde(skip_serializing_if = "Option::is_none")]
  pub quote_order_qty: Option<Decimal>,
  /// Order price.
  #[serde(skip_serializing_if = "Option::is_none")]
  pub price: Option<Decimal>,
  /// Client-specified unique order identifier.
  #[serde(skip_serializing_if = "Option::is_none")]
  pub new_client_order_id: Option<ClientOrderIdTy>,
  /// Stop price for STOP/STOP_MARKET or TAKE_PROFIT/TAKE_PROFIT_MARKET orders.
  #[serde(skip_serializing_if = "Option::is_none")]
  pub stop_price: Option<Decimal>,
  /// See [`CentralizedSignParams`].
  #[serde(flatten)]
  #[serde(skip_serializing_if = "Option::is_none")]
  pub cex_sign_params: Option<CexSignParams>,
}

/// Structure returned when creating orders
#[derive(Debug, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct OrderPostResParams {
  /// Trading pair symbol.
  pub symbol: PairName,
  /// Exchange-assigned order identifier.
  pub order_id: u64,
  /// Client-specified order identifier.
  pub client_order_id: ClientOrderIdTy,
  /// Last update timestamp in milliseconds.
  pub update_time: u64,
  /// Order price.
  pub price: Decimal,
  /// Average fill price.
  pub avg_price: Decimal,
  /// Original order quantity.
  pub orig_qty: Decimal,
  /// Cumulative filled quantity.
  pub cum_qty: Decimal,
  /// Quantity that has been executed.
  pub executed_qty: Decimal,
  /// Cumulative quote asset transacted.
  pub cum_quote: Decimal,
  /// See [`OrderStatus`].
  pub status: OrderStatus,
  /// See [`TimeInForce`].
  pub time_in_force: TimeInForce,
  /// Stop price for stop orders.
  #[serde(skip_serializing_if = "Option::is_none")]
  pub stop_price: Option<Decimal>,
  /// Original order type. See [`OrderType`].
  pub orig_type: OrderType,
  /// Current order type. See [`OrderType`].
  #[serde(rename = "type")]
  pub ty: OrderType,
  /// See [`OrderSide`].
  pub side: OrderSide,
}

#[wtx::pkg(data_format(verbatim), id(crate::exchange::aster::AsterId), transport(http))]
pub(crate) mod pkg {
  use crate::exchange::aster::{Aster, HttpPkgsAux, OrderPostReqParams, OrderPostResParams};
  use wtx::{client_api_framework::pkg::PkgsAux, misc::LeaseMut};

  #[pkg::aux]
  impl<A, DRSR> HttpPkgsAux<A, DRSR>
  where
    A: LeaseMut<Aster>,
  {
    #[pkg::aux_data]
    fn order_post_data(&mut self, params: &OrderPostReqParams) -> crate::Result<()> {
      let PkgsAux { api, bytes_buffer, send_bytes_buffer, tp, .. } = &mut self.0;
      api.lease().auth_req::<true, _>(
        bytes_buffer,
        Some(params),
        if api.lease().is_dex {
          format_args!("/api/v3/order")
        } else {
          format_args!("/api/v1/order")
        },
        send_bytes_buffer,
        None,
        tp,
      )
    }
  }

  #[pkg::req_data]
  pub type OrderPostReq = ();

  #[pkg::res_data]
  pub type OrderPostRes = OrderPostResParams;
}
