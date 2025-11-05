use crate::exchange::hyperliquid::BuilderInfo;
use rust_decimal::Decimal;
use wtx::collection::ArrayStringU8;

#[derive(Debug, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub enum OrderGrouping {
  Na,
  NormalTlsl,
  PositionTlsp,
}

/// Time in force for a limit order.
#[derive(Debug, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub enum OrderTif {
  /// Add-liquidity only.
  Alo,
  /// Limit-on-close
  Loc,
  /// Good 'til canceled.
  Gtc,
}

/// Take profit or stop loss for a trigger order.
#[derive(Debug, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub enum OrderTpsl {
  /// Take profit
  Tp,
  /// Stop loss
  Sl,
}

/// The type of order, which is either a limit or trigger order.
#[derive(Debug, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub enum OrderTy {
  /// See [`OrderLimitParams`].
  Limit(OrderLimitParams),
  /// See [`OrderTriggerParams`].
  Trigger(OrderTriggerParams),
}

/// Spot or perp order
#[derive(Debug, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Order {
  /// The asset's identifier.
  #[serde(rename = "a", alias = "asset")]
  pub asset: u32,
  /// Whether the order is a buy or a sell.
  #[serde(rename = "b", alias = "isBuy")]
  pub is_buy: bool,
  /// The limit price for the order.
  #[serde(rename = "p", alias = "limitPx")]
  pub limit_price: Decimal,
  /// The size of the order.
  #[serde(rename = "s", alias = "sz")]
  pub size: Decimal,
  /// Whether the order is reduce-only.
  #[serde(rename = "r", alias = "reduceOnly", default)]
  pub reduce_only: bool,
  /// See [`OrderTy`].
  #[serde(rename = "t", alias = "orderType")]
  pub ty: OrderTy,
  /// Client Order ID
  #[serde(rename = "c", alias = "cloid", skip_serializing_if = "Option::is_none")]
  pub cloid: Option<ArrayStringU8<34>>,
}

/// Parameters of a limit order.
#[derive(Debug, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct OrderLimitParams {
  /// See [`OrderTif`].
  pub tif: OrderTif,
}

/// Parameters of a trigger order.
#[derive(Debug, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct OrderTriggerParams {
  /// Whether the trigger order should be a market order.
  pub is_market: bool,
  /// /// The price at which the order is triggered.
  pub trigger_price: Decimal,
  /// See [`OrderTpsl`].
  pub tpsl: OrderTpsl,
}

#[derive(Debug, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct BulkOrder<'any> {
  /// See [`Order`].
  pub orders: &'any [Order],
  /// See [`OrderGrouping`].
  pub grouping: OrderGrouping,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub builder: Option<BuilderInfo>,
}
