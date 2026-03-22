use rust_decimal::Decimal;

/// Request parameters for estimating withdrawal fee
#[derive(Debug, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct WithdrawEstimateFeeReqParams<'any> {
  /// Chain ID: 1(ETH), 56(BSC), 42161(Arbi)
  pub chain_id: u32,
  /// Asset symbol to withdraw
  pub asset: &'any str,
}

/// Response containing withdrawal fee estimation details
#[derive(Debug, serde::Deserialize)]
pub struct WithdrawEstimateFeeResParams {
  /// Current token price in USD
  pub token_price: Decimal,
  /// Minimum fee required for withdrawal
  pub gas_cost: Decimal,
  /// Gas cost converted to USD value
  pub gas_usd_value: Decimal,
}

#[wtx::pkg(data_format(verbatim), id(crate::exchange::aster::AsterId), transport(http))]
pub(crate) mod pkg {
  use crate::exchange::aster::{
    Aster, HttpPkgsAux, WithdrawEstimateFeeReqParams, WithdrawEstimateFeeResParams,
  };
  use wtx::{client_api_framework::pkg::PkgsAux, misc::LeaseMut};

  #[pkg::aux]
  impl<A, DRSR> HttpPkgsAux<A, DRSR>
  where
    A: LeaseMut<Aster>,
  {
    #[pkg::aux_data]
    fn withdraw_estimate_fee_data(
      &mut self,
      params: &WithdrawEstimateFeeReqParams<'_>,
    ) -> crate::Result<()> {
      let PkgsAux { api, bytes_buffer, encode_data, tp, .. } = &mut self.0;
      api.lease().auth_req::<false, _>(
        bytes_buffer,
        encode_data,
        Some(params),
        if api.lease().is_dex {
          format_args!("/api/v3/aster/withdraw/estimateFee")
        } else {
          format_args!("/api/v1/aster/withdraw/estimateFee")
        },
        None,
        tp,
      )
    }
  }

  #[pkg::req_data]
  pub type WithdrawEstimateFeeReq = ();

  #[pkg::res_data]
  pub type WithdrawEstimateFeeRes = WithdrawEstimateFeeResParams;
}
