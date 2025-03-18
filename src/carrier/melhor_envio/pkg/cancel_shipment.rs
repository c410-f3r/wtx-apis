#[wtx_macros::pkg(
  data_format(json),
  id(crate::carrier::melhor_envio::MelhorEnvioId),
  transport(http)
)]
pub(crate) mod pkg {
  use crate::carrier::melhor_envio::{
    HttpPkgsAux, MelhorEnvio, MelhorEnvioResult, misc::manage_before_sending,
  };
  use wtx::client_api_framework::network::HttpParams;

  #[pkg::aux]
  impl<A, DRSR> HttpPkgsAux<A, DRSR> {}

  #[pkg::before_sending]
  async fn before_sending(
    api: &mut MelhorEnvio,
    trans_params: &mut HttpParams,
  ) -> crate::Result<()> {
    manage_before_sending(api, format_args!("/api/v2/me/shipment/cancel"), trans_params).await
  }

  #[derive(Debug, serde::Serialize)]
  #[pkg::req_data]
  pub struct CancelShipmentReq<'any> {
    /// ID
    pub id: &'any str,
    /// Reason ID. Usually always "2"
    pub reason_id: &'any str,
    /// Brief description
    pub description: Option<&'any str>,
  }

  #[pkg::res_data]
  pub type CancelShipmentRes<'any> = MelhorEnvioResult<&'any str, ()>;
}
