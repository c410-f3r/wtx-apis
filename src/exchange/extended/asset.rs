use crate::AssetName;
use rust_decimal::{Decimal, RoundingStrategy, prelude::ToPrimitive};
use wtx::collection::ArrayStringU8;

#[derive(Debug, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Asset {
  pub id: u64,
  pub name: AssetName,
  pub precision: u8,
  pub active: bool,
  pub is_collateral: bool,
  pub settlement_external_id: ArrayStringU8<65>,
  pub settlement_resolution: u32,
  pub l1_external_id: ArrayStringU8<0>,
  pub l1_resolution: u8,
}

impl Asset {
  pub fn convert_human_readable_to_stark_quantity(
    &self,
    internal: Decimal,
    rs: RoundingStrategy,
  ) -> Option<i64> {
    let resolution = Decimal::try_new(self.settlement_resolution.into(), 0).ok()?;
    internal.checked_mul(resolution)?.round_dp_with_strategy(0, rs).to_i64()
  }
}
