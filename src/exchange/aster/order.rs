use rust_decimal::Decimal;

use crate::{
  PairName,
  exchange::aster::{CexSignParams, ClientOrderIdTy},
};

/// Response type for new order requests.
#[derive(Debug, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum V1OrderPostResponseTy {
  /// Acknowledgement response with minimal info.
  Ack,
  /// Result response with full order details.
  Result,
}

/// Order side indicating buy or sell direction.
#[derive(Debug, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum OrderSide {
  /// Buy order.
  Buy,
  /// Sell order.
  Sell,
}

/// Current status of an order in its lifecycle.
#[derive(Debug, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum OrderStatus {
  /// Order was canceled.
  Canceled,
  /// Order expired before execution.
  Expired,
  /// Order completely filled.
  Filled,
  /// Order newly created.
  New,
  /// Order partially filled.
  PartiallyFilled,
  /// Order was rejected.
  Rejected,
}

/// Type of order determining execution behavior.
#[derive(Debug, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum OrderType {
  /// Limit order with specified price.
  Limit,
  /// Market order for immediate execution.
  Market,
  /// Stop limit order.
  Stop,
  /// Stop market order.
  StopMarket,
  /// Take profit limit order.
  TakeProfit,
  /// Take profit market order.
  TakeProfitMarket,
  /// Trailing stop market order.
  TrailingStopMarket,
}

/// Position side for hedge mode trading.
#[derive(Debug, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum PositionSide {
  /// One-way mode position.
  Both,
  /// Long position in hedge mode.
  Long,
  /// Short position in hedge mode.
  Short,
}

/// Time in force policy determining order validity duration.
#[derive(Debug, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum TimeInForce {
  /// Fill or Kill - execute immediately and completely or cancel.
  Fok,
  /// Hidden from other participants
  Hidden,
  /// Immediate or Cancel - execute immediately, cancel unfilled portion.
  Ioc,
  /// Good Till Cancel - remain active until filled or canceled.
  Gtc,
  /// Good Till Crossing - post only, cancel if would immediately match.
  Gtx,
}

/// Price type used for triggering conditional orders.
#[derive(Debug, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum WorkingType {
  /// Use contract/last price for triggering.
  ContractPrice,
  /// Use mark price for triggering.
  MarkPrice,
}

/// Structure sent when querying or deleting orders
#[derive(Debug, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct OrderReqParams<'any> {
  /// Market pair
  pub symbol: &'any str,
  /// ID from exchange
  #[serde(skip_serializing_if = "Option::is_none")]
  pub order_id: Option<u64>,
  /// Custom ID created locally
  #[serde(skip_serializing_if = "Option::is_none")]
  pub orig_client_order_id: Option<ClientOrderIdTy>,
  /// See [`SignParams`].
  #[serde(flatten)]
  pub sign_params: Option<CexSignParams>,
}

/// Structure returned when querying or deleting orders
#[derive(Debug, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct OrderResParams {
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
