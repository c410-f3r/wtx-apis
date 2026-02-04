use rust_decimal::Decimal;

use crate::{
  AssetName, PairName,
  exchange::aster::{CexSignParams, OrderSide},
};

/// Request parameters for account trade history endpoint GET /api/v3/userTrades
#[derive(Debug, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct UserTradesReqParams<'any> {
  /// Trading pair symbol (e.g., "BNBUSDT")
  #[serde(skip_serializing_if = "Option::is_none")]
  pub symbol: Option<&'any str>,
  /// Order ID - must be used together with symbol parameter
  #[serde(skip_serializing_if = "Option::is_none")]
  pub order_id: Option<u64>,
  /// Start time in milliseconds - max 7 day interval with endTime
  #[serde(skip_serializing_if = "Option::is_none")]
  pub start_time: Option<u64>,
  /// End time in milliseconds - max 7 day interval with startTime
  #[serde(skip_serializing_if = "Option::is_none")]
  pub end_time: Option<u64>,
  /// Starting trade ID - cannot be sent with startTime or endTime
  #[serde(skip_serializing_if = "Option::is_none")]
  pub from_id: Option<u64>,
  /// Maximum number of results to return (default 500, max 1000)
  #[serde(skip_serializing_if = "Option::is_none")]
  pub limit: Option<u32>,
  /// Receive window in milliseconds for request validity
  pub cex_sign_params: Option<CexSignParams>,
}

/// Individual trade record from account trade history
#[derive(Debug, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UserTradeResParams {
  /// Trading pair symbol
  pub symbol: PairName,
  /// Unique trade ID
  pub id: i64,
  /// Associated order ID
  pub order_id: i64,
  /// Trade side (BUY or SELL)
  pub side: OrderSide,
  /// Trade execution price
  pub price: Decimal,
  /// Trade quantity in base asset
  pub qty: Decimal,
  /// Trade quantity in quote asset
  pub quote_qty: Decimal,
  /// Commission amount charged
  pub commission: Decimal,
  /// Asset used for commission payment
  pub commission_asset: AssetName,
  /// Trade execution timestamp in milliseconds
  pub time: i64,
  /// Counterparty account ID
  pub counterparty_id: i64,
  /// Create/update identifier (can be null)
  pub create_update_id: Option<i64>,
  /// Whether this account was the maker in the trade
  pub maker: bool,
  /// Whether this account was the buyer in the trade
  pub buyer: bool,
}

#[wtx::pkg(data_format(verbatim), id(crate::exchange::aster::AsterId), transport(http))]
pub(crate) mod pkg {
  use crate::exchange::aster::{Aster, HttpPkgsAux, UserTradeResParams, UserTradesReqParams};
  use wtx::{client_api_framework::pkg::PkgsAux, collection::Vector, misc::LeaseMut};

  #[pkg::aux]
  impl<A, DRSR> HttpPkgsAux<A, DRSR>
  where
    A: LeaseMut<Aster>,
  {
    #[pkg::aux_data]
    fn user_trades_data(&mut self, params: &UserTradesReqParams<'_>) -> crate::Result<()> {
      let PkgsAux { api, bytes_buffer, send_bytes_buffer, tp, .. } = &mut self.0;
      api.lease().auth_req::<false, _>(
        bytes_buffer,
        Some(params),
        if api.lease().is_dex {
          format_args!("/api/v3/userTrades")
        } else {
          format_args!("/api/v1/userTrades")
        },
        send_bytes_buffer,
        None,
        tp,
      )
    }
  }

  #[pkg::req_data]
  pub type UserTradesReq = ();

  #[pkg::res_data]
  pub type UserTradesRes = Vector<UserTradeResParams>;
}
