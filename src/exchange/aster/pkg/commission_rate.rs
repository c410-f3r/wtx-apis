use crate::PairString;
use rust_decimal::Decimal;

/// Market fees
#[derive(Debug, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CommissionRateResParams {
  /// Pair name
  pub symbol: PairString,
  /// Maker commission rate
  pub maker_commission_rate: Decimal,
  /// Taker commission rate
  pub taker_commission_rate: Decimal,
}

#[wtx::pkg(data_format(json), id(crate::exchange::aster::AsterId), transport(http))]
pub(crate) mod pkg {
  use crate::exchange::aster::{Aster, CommissionRateResParams, HttpPkgsAux, MarketReqParams};
  use wtx::{client_api_framework::pkg::PkgsAux, misc::LeaseMut};

  #[pkg::aux]
  impl<A, DRSR> HttpPkgsAux<A, DRSR>
  where
    A: LeaseMut<Aster>,
  {
    #[pkg::aux_data]
    fn commission_rate_data(&mut self, params: &MarketReqParams<'_>) -> crate::Result<()> {
      let PkgsAux { api, bytes_buffer, encode_data, tp, .. } = &mut self.0;
      api.lease().auth_req::<false, _>(
        bytes_buffer,
        encode_data,
        Some(params),
        if api.lease().is_dex {
          format_args!("/api/v3/commissionRate")
        } else {
          format_args!("/api/v1/commissionRate")
        },
        None,
        tp,
      )
    }
  }

  #[pkg::req_data]
  pub type CommissionRateReq = ();

  #[pkg::res_data]
  pub type CommissionRateRes = CommissionRateResParams;
}
