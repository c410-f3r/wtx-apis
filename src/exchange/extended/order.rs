use crate::PairName;
use rust_decimal::Decimal;
use wtx::collection::ArrayStringU8;

#[derive(Clone, Copy, Debug, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum OrderPriceType {
  Market,
  Limit,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum OrderSide {
  Buy,
  Sell,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum OrderStatus {
  New,
  Untriggered,
  PartiallyFilled,
  Filled,
  Cancelled,
  Expired,
  Rejected,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum OrderStatusReason {
  Unknown,
  None,
  UnknownMarket,
  DisabledMarket,
  NotEnoughFunds,
  NoLiquidity,
  InvalidFee,
  InvalidQty,
  InvalidPrice,
  InvalidValue,
  UnknownAccount,
  SelfTradeProtection,
  PostOnlyFailed,
  ReduceOnlyFailed,
  InvalidExpireTime,
  PositionTpslConflict,
  InvalidLeverage,
  PrevOrderNotFound,
  PrevOrderTriggered,
  TpslOtherSideFilled,
  PrevOrderConflict,
  OrderReplaced,
  PostOnlyMode,
  ReduceOnlyMode,
  TradingOffMode,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum OrderTpslType {
  Order,
  Position,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum OrderTriggerDirection {
  Up,
  Down,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum OrderTriggerPriceType {
  Mark,
  Index,
  Last,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum OrderType {
  Limit,
  Conditional,
  Market,
  Tpsl,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum SelfTradeProtectionLevel {
  Disabled,
  Account,
  Client,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum TimeInForce {
  Gtt,
  Ioc,
  Fok,
}

#[derive(Debug, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateOrderConditionalTrigger {
  pub trigger_price: Decimal,
  pub trigger_price_type: OrderTriggerPriceType,
  pub direction: OrderTriggerDirection,
  pub execution_price_type: OrderPriceType,
}

#[derive(Debug, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateOrderTpslTrigger<S> {
  pub trigger_price: Decimal,
  pub trigger_price_type: OrderTriggerPriceType,
  pub price: Decimal,
  pub price_type: OrderPriceType,
  pub settlement: StarkSettlement<S>,
  pub debugging_amounts: StarkDebuggingOrderAmounts,
}

#[derive(Debug, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct OpenOrder {
  pub id: u64,
  pub account_id: u64,
  pub external_id: ArrayStringU8<80>,
  pub market: PairName,
  #[serde(rename = "type")]
  pub order_type: OrderType,
  pub side: OrderSide,
  pub status: OrderStatus,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub status_reason: Option<OrderStatusReason>,
  pub price: Option<Decimal>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub average_price: Option<Decimal>,
  pub qty: Decimal,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub filled_qty: Option<Decimal>,
  pub reduce_only: bool,
  pub post_only: bool,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub payed_fee: Option<Decimal>,
  pub created_time: i64,
  pub updated_time: i64,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub expiry_time: Option<i64>,
}

#[derive(Debug, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PerpetualOrder<S> {
  pub id: S,
  pub market: PairName,
  #[serde(rename = "type")]
  pub order_type: OrderType,
  pub side: OrderSide,
  pub qty: Decimal,
  pub price: Decimal,
  pub reduce_only: bool,
  pub post_only: bool,
  pub time_in_force: TimeInForce,
  pub expiry_epoch_millis: i64,
  pub fee: Decimal,
  pub nonce: u32,
  pub self_trade_protection_level: SelfTradeProtectionLevel,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub cancel_id: Option<S>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub settlement: Option<StarkSettlement<S>>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub trigger: Option<CreateOrderConditionalTrigger>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub tp_sl_type: Option<OrderTpslType>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub take_profit: Option<CreateOrderTpslTrigger<S>>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub stop_loss: Option<CreateOrderTpslTrigger<S>>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub debugging_amounts: Option<StarkDebuggingOrderAmounts>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub builder_fee: Option<Decimal>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub builder_id: Option<i32>,
}

#[derive(Debug, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RegisteredOrder<S> {
  pub id: u64,
  pub external_id: S,
}

#[derive(Debug, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SettlementSignature<S> {
  pub r: S,
  pub s: S,
}

#[derive(Debug, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct StarkDebuggingOrderAmounts {
  pub collateral_amount: i64,
  pub fee_amount: i64,
  pub synthetic_amount: i64,
}

#[derive(Debug, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct StarkSettlement<S> {
  pub signature: SettlementSignature<S>,
  pub stark_key: S,
  pub collateral_position: S,
}
