#[wtx::pkg(data_format(json), id(crate::exchange::extended::ExtendedId), transport(http))]
pub(crate) mod pkg {
  use crate::exchange::extended::{Extended, Fees, HttpPkgsAux, HttpResponse};
  use wtx::{client_api_framework::network::HttpParams, collection::Vector};

  #[pkg::aux]
  impl<A, DRSR> HttpPkgsAux<A, DRSR> {}

  #[pkg::before_sending]
  async fn before_sending(
    api: &mut Extended,
    params: &mut V1UserFeesGetParams<'_>,
    trans_params: &mut HttpParams,
  ) -> crate::Result<()> {
    api.auth_req(format_args!("/v1/user/fees?market={params}"), trans_params)?;
    Ok(())
  }

  #[pkg::params]
  pub type V1UserFeesGetParams<'any> = &'any str;

  #[derive(Debug, serde::Serialize)]
  #[pkg::req_data]
  pub struct V1UserFeesGetReq;

  #[pkg::res_data]
  pub type V1UserFeesGetRes = HttpResponse<Vector<Fees>>;
}
