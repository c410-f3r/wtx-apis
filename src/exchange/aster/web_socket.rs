use crate::PairName;
use rust_decimal::Decimal;

/// WebSocket Event
#[derive(Debug, serde::Deserialize)]
pub enum WebSocketEvent {
  /// See [`BookTicker`].
  BookTicker,
}

/// Best bid or ask's price or quantity in real-time.
#[derive(Debug, serde::Deserialize)]
pub struct BookTicker {
  /// See [`WebSocketEvent`].
  #[serde(rename = "e")]
  pub event_type: WebSocketEvent,
  /// Order‑book update identifier
  #[serde(rename = "u")]
  pub update_id: u64,
  /// Event timestamp (ms since Unix epoch)
  #[serde(rename = "E")]
  pub event_timestamp: u64,
  /// Transaction timestamp (ms since Unix epoch)
  #[serde(rename = "T")]
  pub transaction_timestamp: u64,
  /// Symbol, e.g. `"BNBUSDT"`
  #[serde(rename = "s")]
  pub symbol: PairName,
  /// Best bid price (high‑precision decimal)
  #[serde(rename = "b")]
  pub best_bid_price: Decimal,
  /// Best bid quantity
  #[serde(rename = "B")]
  pub best_bid_qty: Decimal,
  /// Best ask price (high‑precision decimal)
  #[serde(rename = "a")]
  pub best_ask_price: Decimal,
  /// Best ask quantity
  #[serde(rename = "A")]
  pub best_ask_qty: Decimal,
}
