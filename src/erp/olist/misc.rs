use crate::erp::olist::Olist;
use wtx::{
  client_api_framework::network::{HttpParams, transport::TransportParams},
  http::ReqBuilder,
  misc::UriString,
};

pub(crate) async fn manage_before_sending(
  api: &mut Olist,
  trans_params: &mut HttpParams,
  cb: impl FnOnce(&mut UriString) -> wtx::Result<()>,
) -> crate::Result<()> {
  api.common.manage_access_token().await;
  cb(&mut trans_params.ext_req_params_mut().uri)?;
  let headers = &mut trans_params.ext_req_params_mut().headers;
  let _ = ReqBuilder::get(headers).auth_bearer(format_args!("{}", &api.common.access_token))?;
  Ok(())
}
