use crate::carrier::super_frete::SuperFrete;
use wtx::{
  client_api_framework::network::{HttpParams, transport::TransportParams as _},
  http::{Method, ReqBuilder},
};

pub(crate) fn manage_token(
  api: &mut SuperFrete,
  endpoint: &str,
  trans_params: &mut HttpParams,
) -> crate::Result<()> {
  let headers = &mut trans_params.ext_req_params_mut().rrb.headers;
  let _ = ReqBuilder::get(headers).auth_bearer(format_args!("{}", &api.token))?;
  trans_params.ext_req_params_mut().method = Method::Post;
  trans_params.ext_req_params_mut().rrb.uri.push_path(format_args!("{endpoint}"))?;
  Ok(())
}
