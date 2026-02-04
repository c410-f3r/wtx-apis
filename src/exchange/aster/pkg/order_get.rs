use crate::{
  PairName,
  exchange::aster::{
    CexSignParams, ClientOrderIdTy, OrderSide, OrderStatus, OrderType, TimeInForce,
  },
};
use rust_decimal::Decimal;

/// Structure sent when querying orders
#[derive(Debug, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct OrderGetReqParams {
  /// Market pair
  pub symbol: PairName,
  /// ID from exchange
  #[serde(skip_serializing_if = "Option::is_none")]
  pub order_id: Option<u64>,
  /// Custom ID created locally
  #[serde(skip_serializing_if = "Option::is_none")]
  pub orig_client_order_id: Option<ClientOrderIdTy>,
  /// See [`SignParams`].
  #[serde(flatten)]
  pub sign_params: CexSignParams,
}

/// Structure returned when querying orders
#[derive(Debug, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct OrderGetResParams {
  /// Exchange-assigned order identifier.
  pub order_id: u64,
  /// Trading pair symbol.
  pub symbol: PairName,
  /// See [`OrderStatus`].
  pub status: OrderStatus,
  /// Client-specified order identifier.
  pub client_order_id: ClientOrderIdTy,
  /// Order price.
  pub price: Decimal,
  /// Average fill price.
  pub avg_price: Decimal,
  /// Original order quantity.
  pub orig_qty: Decimal,
  /// Quantity that has been executed.
  pub executed_qty: Decimal,
  /// Cumulative quote asset transacted.
  pub cum_quote: Decimal,
  /// See [`TimeInForce`].
  pub time_in_force: TimeInForce,
  /// Current order type. See [`OrderType`].
  #[serde(rename = "type")]
  pub ty: OrderType,
  /// See [`OrderSide`].
  pub side: OrderSide,
  /// Stop price for stop orders (ignore when order type is TRAILING_STOP_MARKET).
  #[serde(skip_serializing_if = "Option::is_none")]
  pub stop_price: Option<Decimal>,
  /// Original order type. See [`OrderType`].
  pub orig_type: OrderType,
  /// Order creation timestamp in milliseconds.
  pub time: u64,
  /// Last update timestamp in milliseconds.
  pub update_time: u64,
}

#[wtx::pkg(data_format(json), id(crate::exchange::aster::AsterId), transport(http))]
pub(crate) mod pkg {
  use crate::exchange::aster::{Aster, HttpPkgsAux, OrderGetReqParams, OrderGetResParams};
  use wtx::{client_api_framework::pkg::PkgsAux, misc::LeaseMut};

  #[pkg::aux]
  impl<A, DRSR> HttpPkgsAux<A, DRSR>
  where
    A: LeaseMut<Aster>,
  {
    #[pkg::aux_data]
    fn order_get_data(&mut self, params: &OrderGetReqParams) -> crate::Result<()> {
      let PkgsAux { api, bytes_buffer, send_bytes_buffer, tp, .. } = &mut self.0;
      api.lease().auth_req::<false, _>(
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
  pub type OrderGetReq = ();

  #[pkg::res_data]
  pub type OrderGetRes = OrderGetResParams;
}
