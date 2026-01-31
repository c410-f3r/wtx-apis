#[wtx::pkg(data_format(json), id(crate::exchange::extended::ExtendedId), transport(http))]
pub(crate) mod pkg {
  use crate::exchange::extended::{Extended, HttpPkgsAux, HttpResponse, OpenOrder};
  use wtx::{client_api_framework::network::HttpParams, collection::Vector};

  #[pkg::aux]
  impl<A, DRSR> HttpPkgsAux<A, DRSR> {}

  #[pkg::before_sending]
  async fn before_sending(api: &mut Extended, trans_params: &mut HttpParams) -> crate::Result<()> {
    api.auth_req(format_args!("/v1/user/orders"), trans_params)?;
    Ok(())
  }

  #[derive(Debug, serde::Serialize)]
  #[pkg::req_data]
  pub struct V1UserOpenOrdersGetReq;

  #[pkg::res_data]
  pub type V1UserOpenOrdersGetRes<'de> = HttpResponse<Vector<OpenOrder>>;
}
