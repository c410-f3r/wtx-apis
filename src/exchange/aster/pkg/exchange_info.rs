use crate::{
  AssetString, PairString,
  exchange::aster::{OrderType, TimeInForce},
};
use rust_decimal::Decimal;
use wtx::collection::{ArrayStringU8, ArrayVectorU8, Vector};

/// Time interval for rate limiting
#[derive(Clone, Copy, Debug, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum RateLimitInterval {
  /// Per second interval
  Second,
  /// Per minute interval
  Minute,
}

/// Type of rate limit applied
#[derive(Clone, Copy, Debug, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum RateLimitType {
  /// Weight-based request limiting
  RequestWeight,
  /// Order count limiting
  Orders,
}

/// Filter types applied to trading symbols
#[derive(Debug, serde::Deserialize, serde::Serialize)]
#[serde(tag = "filterType", rename_all = "SCREAMING_SNAKE_CASE")]
pub enum SymbolFilter {
  /// Price range and tick size filter
  #[serde(rename_all = "camelCase")]
  PriceFilter {
    /// Minimum allowed price
    min_price: Decimal,
    /// Maximum allowed price
    max_price: Decimal,
    /// Minimum price increment
    tick_size: Decimal,
  },
  /// Quantity range and step size filter
  #[serde(rename_all = "camelCase")]
  LotSize {
    /// Minimum order quantity
    min_qty: Decimal,
    /// Maximum order quantity
    max_qty: Decimal,
    /// Minimum quantity increment
    step_size: Decimal,
  },
  /// Market order quantity limits
  #[serde(rename_all = "camelCase")]
  MarketLotSize {
    /// Minimum market order quantity
    min_qty: Decimal,
    /// Maximum market order quantity
    max_qty: Decimal,
    /// Minimum quantity increment for market orders
    step_size: Decimal,
  },
  /// Maximum number of open orders
  #[serde(rename_all = "camelCase")]
  MaxNumOrders {
    /// Maximum open orders allowed
    limit: u32,
  },
  /// Minimum notional value filter (legacy)
  #[serde(rename_all = "camelCase")]
  MinNotional {
    /// Minimum order value in quote asset
    min_notional: Decimal,
  },
  /// Maximum notional value filter
  #[serde(rename_all = "camelCase")]
  MaxNotional {
    /// Maximum order value in quote asset
    max_notional: Decimal,
  },
  /// Combined notional value filter
  #[serde(rename_all = "camelCase")]
  Notional {
    /// Minimum order value
    min_notional: Decimal,
    /// Maximum order value
    max_notional: Option<Decimal>,
    /// Minutes to calculate average price
    avg_price_mins: u32,
    /// Apply minimum to market orders
    apply_min_to_market: bool,
    /// Apply maximum to market orders
    apply_max_to_market: bool,
  },
  /// Price deviation filter from mark price
  #[serde(rename_all = "camelCase")]
  PercentPrice {
    /// Multiplier for minimum allowed price
    multiplier_down: Decimal,
    /// Multiplier for maximum allowed price
    multiplier_up: Decimal,
    /// Decimal places for multiplier
    multiplier_decimal: ArrayStringU8<3>,
  },
  /// Price deviation filter with bid/ask separation
  #[serde(rename_all = "camelCase")]
  PercentPriceBySide {
    /// Multiplier for maximum bid price
    bid_multiplier_up: Decimal,
    /// Multiplier for minimum bid price
    bid_multiplier_down: Decimal,
    /// Multiplier for maximum ask price
    ask_multiplier_up: Decimal,
    /// Multiplier for minimum ask price
    ask_multiplier_down: Decimal,
    /// Minutes to calculate average price
    avg_price_mins: u32,
    /// Decimal places for multiplier
    multiplier_decimal: ArrayStringU8<3>,
  },
}

/// Trading status of a symbol
#[derive(Clone, Copy, Debug, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum SymbolStatus {
  /// Symbol is actively trading
  Trading,
  /// Symbol has been delisted
  Delisted,
}

/// Asset information
#[derive(Debug, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Asset {
  /// Asset ticker symbol
  pub asset: AssetString,
}

/// Root response from the exchange info endpoint
#[derive(Debug, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ExchangeInfo {
  /// Server timezone (e.g., "UTC")
  pub timezone: ArrayStringU8<8>,
  /// Server time in milliseconds since epoch
  pub server_time: u64,
  /// Rate limit configurations for the exchange
  pub rate_limits: Vector<RateLimit>,
  /// See [`Asset`].
  pub assets: Vector<Asset>,
  /// See [`Symbol`].
  pub symbols: Vector<Symbol>,
}

/// Rate limit configuration for API requests
#[derive(Debug, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RateLimit {
  /// See [`RateLimitType`].
  pub rate_limit_type: RateLimitType,
  /// See [`RateLimitInterval`].
  pub interval: RateLimitInterval,
  /// Number of intervals
  pub interval_num: u32,
  /// Maximum allowed requests/orders in the interval
  pub limit: u32,
}

/// Trading symbol/pair configuration
#[derive(Debug, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Symbol {
  /// See [`SymbolStatus`].
  pub status: SymbolStatus,
  /// Base asset of the trading pair
  pub base_asset: AssetString,
  /// Quote asset of the trading pair
  pub quote_asset: AssetString,
  /// Decimal precision for price
  pub price_precision: u8,
  /// Decimal precision for quantity
  pub quantity_precision: u8,
  /// Decimal precision for base asset
  pub base_asset_precision: u8,
  /// Decimal precision for quote asset
  pub quote_precision: u8,
  /// Timestamp when the symbol was listed (ms)
  pub listing_time: u64,
  /// Blockchain address of the base asset (if applicable)
  pub base_asset_address: Option<ArrayStringU8<42>>,
  /// Trading filters applied to this symbol
  pub filters: Vector<SymbolFilter>,
  /// Allowed order types for this symbol
  pub order_types: ArrayVectorU8<OrderType, 8>,
  /// Allowed time-in-force options
  pub time_in_force: ArrayVectorU8<TimeInForce, 8>,
  /// Combined symbol name (e.g., "BTCUSDT")
  pub symbol: PairString,
  /// Whether OCO orders are allowed
  pub oco_allowed: bool,
}

#[wtx::pkg(data_format(json), id(crate::exchange::aster::AsterId), transport(http))]
pub(crate) mod pkg {
  use crate::exchange::aster::{Aster, ExchangeInfo, HttpPkgsAux};
  use wtx::client_api_framework::network::{HttpParams, transport::TransportParams};

  #[pkg::aux]
  impl<A, DRSR> HttpPkgsAux<A, DRSR> {}

  #[pkg::before_sending]
  async fn before_sending(api: &mut Aster, trans_params: &mut HttpParams) -> crate::Result<()> {
    trans_params.ext_req_params_mut().rrb.uri.push_path(if api.is_dex {
      format_args!("/api/v3/exchangeInfo")
    } else {
      format_args!("/api/v1/exchangeInfo")
    })?;
    Ok(())
  }

  #[pkg::req_data]
  pub type ExchangeInfoReq = ();

  #[pkg::res_data]
  pub type ExchangeInfoRes = ExchangeInfo;
}
