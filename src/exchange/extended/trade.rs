use crate::{PairName, exchange::extended::OrderSide};
use rust_decimal::Decimal;

#[derive(Clone, Copy, Debug, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum TradeType {
  Deleverage,
  Liquidation,
  Trade,
}

#[derive(Debug, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AccountTrade {
  pub id: i64,
  pub account_id: i64,
  pub market: PairName,
  pub order_id: i64,
  pub side: OrderSide,
  pub price: Decimal,
  pub qty: Decimal,
  pub value: Decimal,
  pub fee: Decimal,
  pub is_taker: bool,
  pub trade_type: TradeType,
  pub created_time: i64,
}

#[derive(Debug, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PublicTrade {
  #[serde(rename = "i")]
  pub id: i64,
  #[serde(rename = "m")]
  pub market: PairName,
  #[serde(rename = "S")]
  pub side: OrderSide,
  #[serde(rename = "tT")]
  pub trade_type: TradeType,
  #[serde(rename = "T")]
  pub timestamp: i64,
  #[serde(rename = "p")]
  pub price: Decimal,
  #[serde(rename = "q")]
  pub qty: Decimal,
}
