#[wtx::pkg(data_format(json), id(crate::exchange::extended::ExtendedId), transport(http))]
pub(crate) mod pkg {
  use crate::exchange::extended::{Extended, HttpPkgsAux, HttpResponse, OpenOrder};
  use wtx::client_api_framework::network::HttpParams;

  #[pkg::aux]
  impl<A, DRSR> HttpPkgsAux<A, DRSR> {}

  #[pkg::before_sending]
  async fn before_sending(
    api: &mut Extended,
    params: &mut V1UserOrderGetParams,
    trans_params: &mut HttpParams,
  ) -> crate::Result<()> {
    api.auth_req(format_args!("/v1/user/orders/{params}"), trans_params)?;
    Ok(())
  }

  #[pkg::params]
  pub type V1UserOrderGetParams = u64;

  #[derive(Debug, serde::Serialize)]
  #[pkg::req_data]
  pub struct V1UserOrderGetReq;

  #[pkg::res_data]
  pub type V1UserOrderGetRes<'de> = HttpResponse<OpenOrder>;
}
