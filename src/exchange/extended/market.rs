use crate::{AssetName, PairName, exchange::extended::Asset};
use rust_decimal::Decimal;
use wtx::collection::{ArrayStringU8, ArrayVectorU8};

#[derive(Debug, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct L2Config {
  // `type` is a reserved keyword in Rust, so we rename the field
  // and use `serde(rename)` to match the original field name.
  #[serde(rename = "type")]
  pub ty: ArrayStringU8<8>,
  pub collateral_id: ArrayStringU8<65>,
  pub collateral_resolution: u32,
  pub synthetic_id: ArrayStringU8<65>,
  pub synthetic_resolution: u32,
}

#[derive(Debug, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Market {
  pub name: PairName,
  pub asset_name: AssetName,
  pub asset_precision: u8,
  pub collateral_asset_name: AssetName,
  pub collateral_asset_precision: u8,
  pub active: bool,
  pub market_stats: MarketStats,
  pub trading_config: TradingConfig,
  pub l2_config: L2Config,
}

impl Market {
  pub fn synthetic_asset(&self) -> Asset {
    Asset {
      id: 1,
      name: self.asset_name,
      precision: self.asset_precision,
      active: self.active,
      is_collateral: false,
      settlement_external_id: self.l2_config.synthetic_id,
      settlement_resolution: self.l2_config.synthetic_resolution,
      l1_external_id: ArrayStringU8::default(),
      l1_resolution: 0,
    }
  }

  pub fn collateral_asset(&self) -> Asset {
    Asset {
      id: 2, // Hardcoded in Python
      name: self.collateral_asset_name.clone(),
      precision: self.collateral_asset_precision,
      active: self.active,
      is_collateral: true,
      settlement_external_id: self.l2_config.collateral_id.clone(),
      settlement_resolution: self.l2_config.collateral_resolution,
      l1_external_id: ArrayStringU8::default(),
      l1_resolution: 0,
    }
  }
}

#[derive(Debug, serde::Deserialize, serde::Serialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct MarketStats {
  pub daily_volume: Decimal,
  pub daily_volume_base: Decimal,
  pub daily_price_change: Decimal,
  pub daily_low: Decimal,
  pub daily_high: Decimal,
  pub last_price: Decimal,
  pub ask_price: Decimal,
  pub bid_price: Decimal,
  pub mark_price: Decimal,
  pub index_price: Decimal,
  pub funding_rate: Decimal,
  pub next_funding_rate: i64,
  pub open_interest: Decimal,
  pub open_interest_base: Decimal,
}

#[derive(Debug, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RiskFactorConfig {
  pub upper_bound: Decimal,
  pub risk_factor: Decimal,
}

#[derive(Debug, serde::Deserialize, serde::Serialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct TradingConfig {
  pub min_order_size: Decimal,
  pub min_order_size_change: Decimal,
  pub min_price_change: Decimal,
  pub max_market_order_value: Decimal,
  pub max_limit_order_value: Decimal,
  pub max_position_value: Decimal,
  pub max_leverage: Decimal,
  pub max_num_orders: ArrayStringU8<8>,
  pub limit_price_cap: Decimal,
  pub limit_price_floor: Decimal,
  pub risk_factor_config: ArrayVectorU8<RiskFactorConfig, 2>,
}
