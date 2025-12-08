#[wtx::pkg(
  data_format(json),
  id(crate::payment_gateway::mercado_pago::MercadoPagoId),
  transport(http)
)]
pub(crate) mod pkg {
  use crate::{
    misc::{OauthRequest, OauthResponse},
    payment_gateway::mercado_pago::PkgsAux,
  };
  use wtx::{
    client_api_framework::network::{HttpParams, transport::TransportParams},
    http::Method,
  };

  #[pkg::aux]
  impl<A, DRSR, TP> PkgsAux<A, DRSR, TP> where TP: TransportParams {}

  #[pkg::before_sending]
  async fn before_sending(trans_params: &mut HttpParams) -> crate::Result<()> {
    trans_params.ext_req_params_mut().method = Method::Post;
    trans_params.ext_req_params_mut().rrb.uri.push_path(format_args!("/oauth/token"))?;
    Ok(())
  }

  #[derive(Debug, serde::Serialize)]
  #[pkg::req_data]
  pub struct OauthReq<'any> {
    /// Request
    pub request: OauthRequest<'any>,
    /// Test token
    pub test_token: Option<bool>,
  }

  #[pkg::res_data]
  pub type OauthRes<'de> = OauthResponse<&'de str>;
}
