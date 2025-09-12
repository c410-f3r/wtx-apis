#[wtx::pkg(data_format(json), id(crate::erp::olist::OlistId), transport(http))]
pub(crate) mod pkg {
  use crate::erp::olist::{
    ContactPost, HttpPkgsAux, Olist, OlistResult, misc::manage_before_sending,
  };
  use alloc::string::String;
  use wtx::{
    client_api_framework::network::{HttpParams, transport::TransportParams},
    http::Method,
  };

  #[pkg::aux]
  impl<A, DRSR> HttpPkgsAux<A, DRSR> {}

  #[pkg::before_sending]
  async fn before_sending(api: &mut Olist, trans_params: &mut HttpParams) -> crate::Result<()> {
    trans_params.ext_req_params_mut().method = Method::Post;
    manage_before_sending(api, trans_params, |el| el.push_path(format_args!("/contatos"))).await?;
    Ok(())
  }

  #[pkg::req_data]
  pub type PostContactReq<S> = ContactPost<S>;

  #[pkg::res_data]
  pub type PostContactRes = OlistResult<PostContactReturn, String>;

  /// Response
  #[derive(Debug, serde::Deserialize)]
  pub struct PostContactReturn {
    /// ID
    pub id: u32,
  }
}
