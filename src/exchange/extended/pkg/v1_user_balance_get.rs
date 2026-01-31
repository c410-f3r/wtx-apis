#[wtx::pkg(data_format(json), id(crate::exchange::extended::ExtendedId), transport(http))]
pub(crate) mod pkg {
  use crate::exchange::extended::{Balance, Extended, HttpPkgsAux, HttpResponse};
  use wtx::client_api_framework::network::HttpParams;

  #[pkg::aux]
  impl<A, DRSR> HttpPkgsAux<A, DRSR> {}

  #[pkg::before_sending]
  async fn before_sending(api: &mut Extended, trans_params: &mut HttpParams) -> crate::Result<()> {
    api.auth_req(format_args!("/v1/user/balance"), trans_params)?;
    Ok(())
  }

  #[derive(Debug, serde::Serialize)]
  #[pkg::req_data]
  pub struct V1UserBalanceGetReq;

  #[pkg::res_data]
  pub type V1UserBalanceGetRes<'de> = HttpResponse<Balance>;
}
