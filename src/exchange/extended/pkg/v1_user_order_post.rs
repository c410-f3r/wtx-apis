#[wtx::pkg(data_format(json), id(crate::exchange::extended::ExtendedId), transport(http))]
pub(crate) mod pkg {
  use crate::exchange::extended::{
    Extended, HttpPkgsAux, HttpResponse, PerpetualOrder, RegisteredOrder,
  };
  use wtx::{
    client_api_framework::network::{HttpParams, transport::TransportParams},
    http::Method,
  };

  #[pkg::aux]
  impl<A, DRSR> HttpPkgsAux<A, DRSR> {}

  #[pkg::before_sending]
  async fn before_sending(api: &mut Extended, trans_params: &mut HttpParams) -> crate::Result<()> {
    api.auth_req(format_args!("/v1/user/order"), trans_params)?;
    trans_params.ext_req_params_mut().method = Method::Post;
    Ok(())
  }

  #[pkg::req_data]
  pub type V1UserOrderPostReq<'any> = PerpetualOrder<&'any str>;

  #[pkg::res_data]
  pub type V1UserOrderPostRes<'de> = HttpResponse<RegisteredOrder<&'de str>>;
}
