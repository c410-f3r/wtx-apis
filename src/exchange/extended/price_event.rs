use crate::PairName;
use rust_decimal::Decimal;

#[derive(Debug, serde::Deserialize)]
pub struct PriceEvent {
  pub m: PairName,
  pub p: Decimal,
  pub ts: u64,
}
