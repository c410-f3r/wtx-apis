#[wtx::pkg(data_format(json), id(crate::exchange::extended::ExtendedId), transport(http))]
pub(crate) mod pkg {
  use crate::exchange::extended::{Extended, HttpPkgsAux, HttpResponse};
  use wtx::{
    client_api_framework::network::{HttpParams, transport::TransportParams},
    http::Method,
  };

  #[pkg::aux]
  impl<A, DRSR> HttpPkgsAux<A, DRSR> {}

  #[pkg::before_sending]
  async fn before_sending(api: &mut Extended, trans_params: &mut HttpParams) -> crate::Result<()> {
    api.auth_req(format_args!("/v1/user/order/massCancel"), trans_params)?;
    trans_params.ext_req_params_mut().method = Method::Post;
    Ok(())
  }

  #[derive(Debug, serde::Serialize)]
  #[pkg::req_data]
  #[serde(rename_all = "camelCase")]
  pub struct V1UserOrderMassCancelPostReq<'any> {
    #[serde(skip_serializing_if = "Option::is_none")]
    order_ids: Option<&'any [&'any str]>,
    #[serde(skip_serializing_if = "Option::is_none")]
    external_order_ids: Option<&'any [&'any str]>,
    #[serde(skip_serializing_if = "Option::is_none")]
    markets: Option<&'any [&'any str]>,
    #[serde(skip_serializing_if = "Option::is_none")]
    cancel_all: Option<bool>,
  }

  #[pkg::res_data]
  pub type V1UserOrderPostRes<'de> = HttpResponse<Nothing>;

  #[derive(Debug, serde::Deserialize)]
  pub struct Nothing {}
}
