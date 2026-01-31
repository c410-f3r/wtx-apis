#[wtx::pkg(data_format(json), id(crate::exchange::extended::ExtendedId), transport(http))]
pub(crate) mod pkg {
  use crate::exchange::extended::{Extended, HttpPkgsAux, HttpResponse};
  use wtx::{
    client_api_framework::network::{HttpParams, transport::TransportParams},
    http::Method,
  };

  #[pkg::aux]
  impl<A, DRSR> HttpPkgsAux<A, DRSR> {}

  #[pkg::before_sending]
  async fn before_sending(
    api: &mut Extended,
    params: &mut V1UserOrderIdDeleteParams<'_>,
    trans_params: &mut HttpParams,
  ) -> crate::Result<()> {
    api.auth_req(format_args!("/v1/user/order/{params}"), trans_params)?;
    trans_params.ext_req_params_mut().method = Method::Delete;
    Ok(())
  }

  #[pkg::params]
  pub type V1UserOrderIdDeleteParams<'any> = &'any str;

  #[derive(Debug, serde::Serialize)]
  #[pkg::req_data]
  pub struct V1UserOrderIdDeleteReq;

  #[pkg::res_data]
  pub type V1UserOrderIdDeleteRes = HttpResponse<Nothing>;

  #[derive(Debug, serde::Deserialize)]
  pub struct Nothing {}
}
