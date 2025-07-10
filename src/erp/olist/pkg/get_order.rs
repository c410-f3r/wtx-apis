#[wtx::pkg(data_format(json), id(crate::erp::olist::OlistId), transport(http))]
pub(crate) mod pkg {
  use crate::erp::olist::{HttpPkgsAux, Olist, OlistResult, OrderGet, misc::manage_before_sending};
  use alloc::string::String;
  use wtx::client_api_framework::network::HttpParams;

  #[pkg::aux]
  impl<A, DRSR> HttpPkgsAux<A, DRSR> {}

  #[pkg::before_sending]
  async fn before_sending(
    api: &mut Olist,
    params: &mut GetOrderParams<'_>,
    trans_params: &mut HttpParams,
  ) -> crate::Result<()> {
    manage_before_sending(api, trans_params, |el| {
      el.push_path(format_args!("/pedidos/{}", params.id))
    })
    .await?;
    Ok(())
  }

  #[derive(Debug)]
  #[pkg::params]
  pub struct GetOrderParams<'any> {
    id: &'any str,
  }

  #[derive(Debug, serde::Serialize)]
  #[pkg::req_data]
  pub struct GetOrderReq;

  #[pkg::res_data]
  pub type GetOrderRes<'any> = OlistResult<OrderGet<String>, String>;
}
