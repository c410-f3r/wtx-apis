#[wtx::pkg(data_format(json), id(crate::carrier::super_frete::SuperFreteId), transport(http))]
pub(crate) mod pkg {
  use crate::carrier::super_frete::{
    HttpPkgsAux, SendFreightReqGeneric, SendFreightResGeneric, SuperFrete, SuperFreteResponse,
    misc::manage_token,
  };
  use wtx::client_api_framework::network::HttpParams;

  #[pkg::aux]
  impl<A, DRSR> HttpPkgsAux<A, DRSR> {}

  #[pkg::before_sending]
  async fn before_sending(
    api: &mut SuperFrete,
    trans_params: &mut HttpParams,
  ) -> crate::Result<()> {
    manage_token(api, "/api/v0/cart", trans_params)
  }

  #[pkg::req_data]
  pub type SendFreightReq<P, S> = SendFreightReqGeneric<P, S>;

  #[pkg::res_data]
  pub type SendFreightRes<'any> = SuperFreteResponse<&'any str, SendFreightResGeneric<&'any str>>;
}
