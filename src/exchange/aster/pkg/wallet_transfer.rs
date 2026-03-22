use rust_decimal::Decimal;

/// Transfer status
#[derive(Debug, serde::Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum WalletTransferStatus {
  /// Successful transfer
  Success,
}

/// Transfer type
#[derive(Debug, serde::Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum WalletTransferTy {
  /// Future to Spot
  FutureSpot,
  /// Spot to Future
  SpotFuture,
}

/// Transfer between inner accounts
#[derive(Debug, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct WalletTransferReqParams<'any> {
  /// Amount
  pub amount: Decimal,
  /// Asset
  pub asset: &'any str,
  /// Client transfer id
  pub client_tran_id: &'any str,
  /// Kind type
  pub kind_type: WalletTransferTy,
}

/// Transfer status response
#[derive(Debug, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WalletTransferResParams {
  /// Transfer id
  pub tran_id: Decimal,
  /// Transfer status
  pub status: WalletTransferStatus,
}

#[wtx::pkg(data_format(verbatim), id(crate::exchange::aster::AsterId), transport(http))]
pub(crate) mod pkg {
  use crate::exchange::aster::{
    Aster, HttpPkgsAux, WalletTransferReqParams, WalletTransferResParams,
  };
  use wtx::{client_api_framework::pkg::PkgsAux, misc::LeaseMut};

  #[pkg::aux]
  impl<A, DRSR> HttpPkgsAux<A, DRSR>
  where
    A: LeaseMut<Aster>,
  {
    #[pkg::aux_data]
    fn wallet_transfer_data(&mut self, params: &WalletTransferReqParams<'_>) -> crate::Result<()> {
      let PkgsAux { api, bytes_buffer, encode_data, tp, .. } = &mut self.0;
      api.lease().auth_req::<true, _>(
        bytes_buffer,
        encode_data,
        Some(params),
        if api.lease().is_dex {
          format_args!("/api/v3/asset/wallet/transfer")
        } else {
          format_args!("/api/v1/asset/wallet/transfer")
        },
        None,
        tp,
      )
    }
  }

  #[pkg::req_data]
  pub type WalletTransferReq = ();

  #[pkg::res_data]
  pub type WalletTransferRes = WalletTransferResParams;
}
