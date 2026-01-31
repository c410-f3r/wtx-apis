use crate::{
  PairName,
  exchange::aster::{
    ClientOrderIdTy, OrderSide, OrderStatus, OrderType, TimeInForce, sign_params::SignParams,
  },
};
use rust_decimal::Decimal;

/// Structure sent when querying orders
#[derive(Debug, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct V1OrderGetReqParams {
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
  pub sign_params: SignParams,
}

/// Structure returned when querying orders
#[derive(Debug, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct V1OrderGetResParams {
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
