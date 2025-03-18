#[wtx_macros::pkg(data_format(json), id(crate::erp::olist::OlistId), transport(http))]
pub(crate) mod pkg {
  use crate::erp::olist::{HttpPkgsAux, Info, Olist, OlistResult, misc::manage_before_sending};
  use alloc::string::String;
  use wtx::client_api_framework::network::HttpParams;

  #[pkg::aux]
  impl<A, DRSR> HttpPkgsAux<A, DRSR> {}

  #[pkg::before_sending]
  async fn before_sending(api: &mut Olist, trans_params: &mut HttpParams) -> crate::Result<()> {
    manage_before_sending(api, trans_params, |el| el.push_path(format_args!("/info"))).await?;
    Ok(())
  }

  #[derive(Debug, serde::Serialize)]
  #[pkg::req_data]
  pub struct GetInfoReq;

  #[pkg::res_data]
  pub type GetInfoRes<'de> = OlistResult<Info<String>, String>;
}
