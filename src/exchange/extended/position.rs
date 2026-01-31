use crate::PairName;
use rust_decimal::Decimal;

#[derive(Clone, Copy, Debug, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum ExitType {
  Trade,
  Liquidation,
  Adl,
}

#[derive(Clone, Copy, Debug, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum PositionSide {
  Long,
  Short,
}

#[derive(Debug, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Position {
  pub id: i64,
  pub account_id: i64,
  pub market: PairName,
  pub side: PositionSide,
  pub leverage: Decimal,
  pub size: Decimal,
  pub value: Decimal,
  pub open_price: Decimal,
  pub mark_price: Decimal,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub liquidation_price: Option<Decimal>,
  pub unrealised_pnl: Decimal,
  pub realised_pnl: Decimal,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub tp_price: Option<Decimal>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub sl_price: Option<Decimal>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub adl: Option<i32>,
  pub created_at: i64,
  pub updated_at: i64,
}

#[derive(Debug, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PositionHistory {
  pub id: i64,
  pub account_id: i64,
  pub market: PairName,
  pub side: PositionSide,
  pub leverage: Decimal,
  pub size: Decimal,
  pub open_price: Decimal,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub exit_type: Option<ExitType>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub exit_price: Option<Decimal>,
  pub realised_pnl: Decimal,
  pub created_time: i64,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub closed_time: Option<i64>,
}
