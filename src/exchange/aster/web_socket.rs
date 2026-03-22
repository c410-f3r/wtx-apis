use crate::{
  AssetString, PairString,
  exchange::aster::{
    ClientOrderIdTy, OrderSide, OrderStatus, OrderType, PositionSide, TimeInForce,
  },
};
use rust_decimal::Decimal;

/// Reason type for the account update event
#[derive(Debug, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum AccountUpdateTy {
  /// Deposit
  Deposit,
  /// Withdraw
  Withdraw,
  /// Order
  Order,
  /// Funding Fee
  FundingFee,
  /// Withdraw Reject
  WithdrawReject,
  /// Adjustment
  Adjustment,
  /// Insurance Clear
  InsuranceClear,
  /// Admin Deposit
  AdminDeposit,
  /// Admin Withdraw
  AdminWithdraw,
  /// Margin Transfer
  MarginTransfer,
  /// Margin Type Change
  MarginTypeChange,
  /// Asset Transfer
  AssetTransfer,
  /// Options Premium Fee
  OptionsPremiumFee,
  /// Options Settle Profit
  OptionsSettleProfit,
  /// Auto Exchange
  AutoExchange,
}
/// Current execution type of an order
#[derive(Debug, serde::Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum ExecutionTy {
  /// New order
  New,
  /// Order canceled
  Canceled,
  /// New order was rejected
  Rejected,
  /// Order had a new fill
  Trade,
  /// Order expired (based on the order's Time In Force parameter)
  Expired,
}

/// Margin type for a position
#[derive(Debug, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "lowercase")]
pub enum MarginType {
  /// Cross
  Cross,
  /// Isolated
  Isolated,
}

/// WebSocket Event
#[derive(Debug, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub enum WebSocketEvent {
  /// See [`AccountUpdateTy`]
  AccountUpdate,
  /// See [`BookTickerEvent`].
  BookTicker,
  /// See [`ExecutionReport`]
  ExecutionReport,
  /// See [`OutboundAccountPosition`]
  OutboundAccountPosition,
}

/// Individual asset balance information
#[derive(Debug, serde::Deserialize, serde::Serialize)]
pub struct BalanceWs {
  /// Asset symbol (e.g., "BTC", "LTC")
  #[serde(rename = "a")]
  pub asset: AssetString,
  /// Available balance for trading/withdrawal
  #[serde(rename = "f")]
  pub free: Decimal,
  /// Balance locked in open orders
  #[serde(rename = "l")]
  pub locked: Decimal,
}

/// Best bid or ask's price or quantity in real-time.
#[derive(Debug, serde::Deserialize)]
pub struct BookTickerEvent {
  /// Order‑book update identifier
  #[serde(rename = "u")]
  pub update_id: u64,
  /// Event timestamp (ms since Unix epoch)
  #[serde(rename = "E")]
  pub event_timestamp: u64,
  /// Transaction timestamp (ms since Unix epoch)
  #[serde(rename = "T")]
  pub transaction_timestamp: u64,
  /// Symbol, e.g. `"BNBUSDT"`
  #[serde(rename = "s")]
  pub symbol: PairString,
  /// Best bid price (high‑precision decimal)
  #[serde(rename = "b")]
  pub best_bid_price: Decimal,
  /// Best bid quantity
  #[serde(rename = "B")]
  pub best_bid_qty: Decimal,
  /// Best ask price (high‑precision decimal)
  #[serde(rename = "a")]
  pub best_ask_price: Decimal,
  /// Best ask quantity
  #[serde(rename = "A")]
  pub best_ask_qty: Decimal,
}

/// Execution report event for order updates
#[derive(Debug, serde::Deserialize)]
pub struct ExecutionReport {
  /// Event time
  #[serde(rename = "E")]
  pub event_time: u64,
  /// Symbol
  #[serde(rename = "s")]
  pub symbol: PairString,
  /// Client order ID
  #[serde(rename = "c")]
  pub client_order_id: ClientOrderIdTy,
  /// Order direction (BUY/SELL)
  #[serde(rename = "S")]
  pub side: OrderSide,
  /// Order type
  #[serde(rename = "o")]
  pub order_type: OrderType,
  /// Time in force
  #[serde(rename = "f")]
  pub time_in_force: TimeInForce,
  /// Order quantity
  #[serde(rename = "q")]
  pub quantity: Decimal,
  /// Order price
  #[serde(rename = "p")]
  pub price: Decimal,
  /// Average price
  #[serde(rename = "ap")]
  pub average_price: Decimal,
  /// Stop price
  #[serde(rename = "P")]
  pub stop_price: Decimal,
  /// Current execution type
  #[serde(rename = "x")]
  pub execution_type: ExecutionTy,
  /// Current order status
  #[serde(rename = "X")]
  pub order_status: OrderStatus,
  /// Order ID
  #[serde(rename = "i")]
  pub order_id: u64,
  /// Last executed quantity
  #[serde(rename = "l")]
  pub last_executed_quantity: Decimal,
  /// Cumulative filled quantity
  #[serde(rename = "z")]
  pub cumulative_filled_quantity: Decimal,
  /// Last executed price
  #[serde(rename = "L")]
  pub last_executed_price: Decimal,
  /// Commission amount
  #[serde(rename = "n")]
  pub commission_amount: Option<Decimal>,
  /// Commission asset
  #[serde(rename = "N")]
  pub commission_asset: Option<AssetString>,
  /// Transaction time
  #[serde(rename = "T")]
  pub transaction_time: u64,
  /// Transaction ID
  #[serde(rename = "t")]
  pub transaction_id: u64,
  /// Is this trade the maker side?
  #[serde(rename = "m")]
  pub is_maker: bool,
  /// Original order type
  #[serde(rename = "ot")]
  pub original_order_type: OrderType,
  /// Order creation time
  #[serde(rename = "O")]
  pub order_creation_time: u64,
  /// Cumulative quote asset transacted quantity
  #[serde(rename = "Z")]
  pub cumulative_quote_quantity: Decimal,
  /// Last quote asset transacted quantity (lastPrice * lastQty)
  #[serde(rename = "Y")]
  pub last_quote_quantity: Decimal,
  /// Quote order quantity
  #[serde(rename = "Q")]
  pub quote_order_quantity: Decimal,
}

/// Outbound account position event
#[derive(Debug, serde::Deserialize)]
pub struct OutboundAccountPosition<B> {
  /// Event time
  #[serde(rename = "E")]
  pub event_time: u64,
  /// Time of last account update
  #[serde(rename = "T")]
  pub last_update_time: u64,
  /// Event reason type
  #[serde(rename = "m")]
  pub event_reason: AccountUpdateTy,
  /// Balances
  #[serde(rename = "B")]
  pub balances: B,
}

/// WebSocket payload
#[derive(Debug, serde::Deserialize)]
pub struct Payload<D, S> {
  /// Stream
  pub stream: S,
  /// Data
  pub data: D,
}

/// Position information for a symbol
#[derive(Debug, serde::Deserialize, serde::Serialize)]
pub struct Position {
  /// Symbol (e.g., "BTCUSDT")
  #[serde(rename = "s")]
  pub symbol: PairString,
  /// Position Amount
  #[serde(rename = "pa")]
  pub position_amount: Decimal,
  /// Entry Price
  #[serde(rename = "ep")]
  pub entry_price: Decimal,
  /// (Pre-fee) Accumulated Realized PnL
  #[serde(rename = "cr")]
  pub accumulated_realized: Decimal,
  /// Unrealized PnL
  #[serde(rename = "up")]
  pub unrealized_pnl: Decimal,
  /// Margin Type
  #[serde(rename = "mt")]
  pub margin_type: MarginType,
  /// Isolated Wallet (if isolated position)
  #[serde(rename = "iw")]
  pub isolated_wallet: Decimal,
  /// Position Side
  #[serde(rename = "ps")]
  pub position_side: PositionSide,
}

/// Generic WebSocket event
#[derive(Debug, serde::Deserialize, serde::Serialize)]
pub struct WsEvent {
  /// See [`WebSocketEvent`].
  #[serde(rename = "e")]
  pub event_type: WebSocketEvent,
}
