use crate::PairName;
use rust_decimal::Decimal;

#[derive(Debug, serde::Deserialize)]
pub struct OrderBookEventData<A, B> {
  #[serde(rename = "m")]
  pub market: PairName,
  #[serde(rename = "b")]
  pub bids: B,
  #[serde(rename = "a")]
  pub asks: A,
}

#[derive(Debug, serde::Deserialize)]
pub struct OrderBookEventLevel {
  #[serde(rename = "q")]
  pub quantity: Decimal,
  #[serde(rename = "p")]
  pub price: Decimal,
}

#[derive(Debug, serde::Deserialize)]
pub struct OrderBookResponse<A, B> {
  pub ask: A,
  pub bid: B,
  pub market: PairName,
}

#[derive(Debug, serde::Deserialize)]
pub struct OrderBookResponseLevel {
  pub price: Decimal,
  pub qty: Decimal,
}
