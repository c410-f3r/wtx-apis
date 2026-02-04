use crate::{AssetName, PairName, exchange::aster::PositionSide};
use rust_decimal::Decimal;
use wtx::collection::Vector;

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
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum WebSocketEvent {
  /// See [`AccountUpdate`]
  AccountUpdate,
  /// See [`BookTicker`].
  BookTicker,
}

/// Account update data containing reason, balances, and positions
#[derive(Debug, serde::Deserialize, serde::Serialize)]
pub struct AccountUpdate {
  /// Event reason type
  #[serde(rename = "m")]
  pub ty: AccountUpdateTy,
  /// Balances
  #[serde(rename = "B")]
  pub balances: Vector<BalanceWs>,
  /// Positions
  #[serde(rename = "P")]
  pub positions: Vector<Position>,
}

/// Balance information for an asset
#[derive(Debug, serde::Deserialize, serde::Serialize)]
pub struct BalanceWs {
  /// Asset name (e.g., "USDT", "BUSD")
  #[serde(rename = "a")]
  pub asset: AssetName,
  /// Wallet Balance
  #[serde(rename = "wb")]
  pub wallet_balance: Decimal,
  /// Cross Wallet Balance
  #[serde(rename = "cw")]
  pub cross_wallet_balance: Decimal,
  /// Balance Change except PnL and Commission
  #[serde(rename = "bc")]
  pub balance_change: Decimal,
}

/// Main ACCOUNT_UPDATE event structure
#[derive(Debug, serde::Deserialize, serde::Serialize)]
pub struct AccountUpdateEvent {
  /// Event timestamp (ms since Unix epoch)
  #[serde(rename = "E")]
  pub event_timestamp: u64,
  /// Transaction timestamp (ms since Unix epoch)
  #[serde(rename = "T")]
  pub transaction_timestamp: u64,
  /// See [`AccountUpdate`]
  #[serde(rename = "a")]
  pub account_update: AccountUpdate,
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
  pub symbol: PairName,
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

/// Position information for a symbol
#[derive(Debug, serde::Deserialize, serde::Serialize)]
pub struct Position {
  /// Symbol (e.g., "BTCUSDT")
  #[serde(rename = "s")]
  pub symbol: PairName,
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
