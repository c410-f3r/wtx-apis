#[wtx::pkg(data_format(json), id(crate::erp::olist::OlistId), transport(http))]
pub(crate) mod pkg {
  use crate::erp::olist::{
    HttpPkgsAux, Olist, OlistResult, misc::manage_before_sending, order_post::OrderPost,
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
    manage_before_sending(api, trans_params, |el| el.push_path(format_args!("/pedidos"))).await?;
    Ok(())
  }

  #[pkg::req_data]
  pub type PostOrderReq<S> = OrderPost<S>;

  #[pkg::res_data]
  pub type PostOrderRes<'de> = OlistResult<PostOrderReturn<&'de str>, String>;

  /// Response
  #[derive(Debug, serde::Deserialize)]
  #[serde(rename_all = "camelCase")]
  pub struct PostOrderReturn<S> {
    /// ID
    pub id: u32,
    /// Order number
    pub numero_pedido: S,
  }
}
