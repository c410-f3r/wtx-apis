use rust_decimal::Decimal;
use wtx::collection::Vector;

/// Request parameters for the depth endpoint
#[derive(Debug, serde::Serialize)]
pub struct DepthReqParams<'any> {
  /// Trading pair symbol
  pub symbol: &'any str,
  /// Number of price levels to return (default: 100, options: 5, 10, 20, 50, 100, 500, 1000)
  pub limit: Option<u32>,
}

/// Response from the depth endpoint containing order book data
#[derive(Debug, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DepthResParams {
  /// Last update ID for the order book
  pub last_update_id: u64,
  /// Message output time (timestamp in milliseconds)
  #[serde(rename = "E")]
  pub event_time: u64,
  /// Transaction time (timestamp in milliseconds)
  #[serde(rename = "T")]
  pub transaction_time: u64,
  /// List of bid orders [price, quantity]
  pub bids: Vector<PriceLevel>,
  /// List of ask orders [price, quantity]
  pub asks: Vector<PriceLevel>,
}

/// Represents a price level in the order book with price and quantity
#[derive(Debug, serde::Deserialize)]
pub struct PriceLevel {
  /// Price at this level
  pub price: Decimal,
  /// Quantity available at this price level
  pub quantity: Decimal,
}

#[wtx::pkg(data_format(json), id(crate::exchange::aster::AsterId), transport(http))]
pub(crate) mod pkg {
  use crate::exchange::aster::{Aster, DepthReqParams, DepthResParams, HttpPkgsAux};
  use wtx::{client_api_framework::pkg::PkgsAux, misc::LeaseMut};

  #[pkg::aux]
  impl<A, DRSR> HttpPkgsAux<A, DRSR>
  where
    A: LeaseMut<Aster>,
  {
    #[pkg::aux_data]
    fn depth_data(&mut self, params: &DepthReqParams<'_>) -> crate::Result<()> {
      let PkgsAux { api, bytes_buffer, send_bytes_buffer, tp, .. } = &mut self.0;
      api.lease().auth_req::<false, _>(
        bytes_buffer,
        Some(params),
        if api.lease().is_dex {
          format_args!("/api/v3/depth")
        } else {
          format_args!("/api/v1/depth")
        },
        send_bytes_buffer,
        None,
        tp,
      )
    }
  }

  #[pkg::req_data]
  pub type DepthReq = ();

  #[pkg::res_data]
  pub type DepthRes = DepthResParams;
}
