use crate::{
  PairName,
  exchange::aster::{
    ClientOrderIdTy, OrderSide, OrderStatus, OrderType, TimeInForce, sign_params::SignParams,
  },
};
use rust_decimal::Decimal;

/// Structure sent when creating orders
#[derive(Debug, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct V1OrderPostReqParams {
  /// Trading pair symbol.
  pub symbol: PairName,
  /// Order side (buy or sell).
  pub side: OrderSide,
  /// See [`OrderType`].
  #[serde(rename = "type")]
  pub ty: OrderType,
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
  /// See [`SignParams`].
  #[serde(flatten)]
  pub sign_params: SignParams,
}

/// Structure returned when creating orders
#[derive(Debug, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct V1OrderPostResParams {
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
