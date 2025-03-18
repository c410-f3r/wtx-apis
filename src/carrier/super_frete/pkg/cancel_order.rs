#[wtx_macros::pkg(
  data_format(json),
  id(crate::carrier::super_frete::SuperFreteId),
  transport(http)
)]
pub(crate) mod pkg {
  use crate::carrier::super_frete::{
    CancelOrderReqOrder, CancelOrderResGeneric, HttpPkgsAux, SuperFrete, SuperFreteResponse,
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
    manage_token(api, "/api/v0/order/cancel", trans_params)
  }

  #[derive(Debug, serde::Serialize)]
  #[pkg::req_data]
  pub struct CancelOrderReq<'any> {
    /// Order
    pub order: CancelOrderReqOrder<'any>,
  }

  #[pkg::res_data]
  pub type CancelOrderRes<'any> = SuperFreteResponse<&'any str, CancelOrderResGeneric<&'any str>>;
}
