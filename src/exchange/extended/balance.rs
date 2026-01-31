use crate::AssetName;
use rust_decimal::Decimal;

#[derive(Debug, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Balance {
  pub collateral_name: AssetName,
  pub balance: Decimal,
  pub equity: Decimal,
  pub available_for_trade: Decimal,
  pub available_for_withdrawal: Decimal,
  pub unrealised_pnl: Decimal,
  pub initial_margin: Decimal,
  pub margin_ratio: Decimal,
  pub updated_time: u64,
}
