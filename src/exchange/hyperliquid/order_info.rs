use crate::{
  blockchain::AssetName,
  exchange::hyperliquid::{Cloid, OrderTif, OrderTy},
};
use rust_decimal::Decimal;

#[derive(Debug, serde::Deserialize, serde::Serialize)]
pub struct BasicOrderInfo {
  pub coin: AssetName,
  pub side: OrderSide,
  pub limit_px: Decimal,
  pub sz: Decimal,
  pub oid: u64,
  pub timestamp: u64,
  pub is_trigger: bool,
  pub trigger_px: Decimal,
  pub is_position_tpsl: bool,
  pub reduce_only: bool,
  pub order_type: OrderTy,
  pub orig_sz: Decimal,
  pub tif: Option<OrderTif>,
  pub cloid: Option<Cloid>,
}

#[derive(Debug, serde::Deserialize)]
pub struct OrderInfo {
  pub order: BasicOrderInfo,
  pub status: OrderStatus,
  pub status_timestamp: u64,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
pub enum OrderSide {
  /// Base
  #[serde(rename = "A")]
  A,
  /// Quote
  #[serde(rename = "B")]
  B,
}
#[derive(Clone, Copy, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub enum OrderStatus {
  /// Order was placed successfully.
  Open,
  /// Order was filled.
  Filled,
  /// Order was cancelled by the user.
  Canceled,
  /// Trigger order was triggered.
  Triggered,
  /// Order was rejected at the time of placement.
  Rejected,
  /// Cancelled because insufficient margin to fill.
  MarginCanceled,
  /// Vault‑only: cancelled due to a user's withdrawal from a vault.
  VaultWithdrawalCanceled,
  /// Cancelled because the order was too aggressive when open interest was at its cap.
  OpenInterestCapCanceled,
  /// Cancelled due to self‑trade prevention.
  SelfTradeCanceled,
  /// Cancelled reduce‑only order that does not reduce the position.
  ReduceOnlyCanceled,
  /// TP/SL only: cancelled because a sibling order was filled.
  SiblingFilledCanceled,
  /// Cancelled due to asset delisting.
  DelistedCanceled,
  /// Cancelled due to liquidation.
  LiquidatedCanceled,
  /// API‑only: cancelled because the scheduled‑cancel deadline (dead‑man's switch) was exceeded.
  ScheduledCancel,
  /// Rejected due to an invalid tick price.
  TickRejected,
  /// Rejected because the order notional was below the minimum.
  MinTradeNtlRejected,
  /// Rejected due to insufficient margin for a perpetual contract.
  PerpMarginRejected,
  /// Rejected because the order was marked reduce‑only but could not be reduced.
  ReduceOnlyRejected,
  /// Rejected due to a post‑only immediate match (bad ALO price).
  BadAloPxRejected,
  /// Rejected because an IOC order could not be matched.
  IocCancelRejected,
  /// Rejected due to an invalid TP/SL price.
  BadTriggerPxRejected,
  /// Rejected because there was no liquidity for a market order.
  MarketOrderNoLiquidityRejected,
  /// Rejected because open interest was at its cap when trying to increase a position.
  PositionIncreaseAtOpenInterestCapRejected,
  /// Rejected because open interest was at its cap when trying to flip a position.
  PositionFlipAtOpenInterestCapRejected,
  /// Rejected because the price was too aggressive at the open‑interest cap.
  TooAggressiveAtOpenInterestCapRejected,
  /// Rejected due to an open‑interest increase beyond the cap.
  OpenInterestIncreaseRejected,
  /// Rejected because the spot balance was insufficient.
  InsufficientSpotBalanceRejected,
  /// Rejected because the price was too far from the oracle.
  OracleRejected,
  /// Rejected because the position would exceed the margin‑tier limit at the current leverage.
  PerpMaxPositionRejected,
}
