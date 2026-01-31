use rust_decimal::Decimal;

#[derive(Debug, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Fees {
  pub maker_fee_rate: Decimal,
  pub taker_fee_rate: Decimal,
  pub builder_fee_rate: Decimal,
}
