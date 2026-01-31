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
