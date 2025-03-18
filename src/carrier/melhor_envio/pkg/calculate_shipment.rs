#[wtx_macros::pkg(
  data_format(json),
  id(crate::carrier::melhor_envio::MelhorEnvioId),
  transport(http)
)]
pub(crate) mod pkg {
  use crate::carrier::melhor_envio::{
    HttpPkgsAux, MelhorEnvio, MelhorEnvioResult,
    calculate_shipment_request::CalculateShipmentRequest,
    calculate_shipment_response::CalculateShipmentResponse, misc::manage_before_sending,
  };
  use wtx::{client_api_framework::network::HttpParams, misc::Vector};

  #[pkg::aux]
  impl<A, DRSR> HttpPkgsAux<A, DRSR> {}

  #[pkg::before_sending]
  async fn before_sending(
    api: &mut MelhorEnvio,
    trans_params: &mut HttpParams,
  ) -> crate::Result<()> {
    manage_before_sending(api, format_args!("/api/v2/me/shipment/calculate"), trans_params).await
  }

  #[pkg::req_data]
  pub type CalculateShipmentReq<'any> = CalculateShipmentRequest<&'any str>;

  #[pkg::res_data]
  pub type CalculateShipmentRes<'any> =
    MelhorEnvioResult<&'any str, Vector<CalculateShipmentResponse<&'any str>>>;
}
