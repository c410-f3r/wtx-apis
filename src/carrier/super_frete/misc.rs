use crate::{carrier::super_frete::SuperFrete, misc::_apply_auth_header};
use wtx::{
  client_api_framework::network::{HttpParams, transport::TransportParams as _},
  http::Method,
};

pub(crate) fn manage_token(
  api: &mut SuperFrete,
  endpoint: &str,
  trans_params: &mut HttpParams,
) -> crate::Result<()> {
  _apply_auth_header(trans_params, &api.token)?;
  trans_params.ext_req_params_mut().method = Method::Post;
  trans_params.ext_req_params_mut().uri.push_path(format_args!("{endpoint}"))?;
  Ok(())
}
