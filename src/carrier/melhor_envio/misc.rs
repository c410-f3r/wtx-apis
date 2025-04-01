use crate::carrier::melhor_envio::MelhorEnvio;
use core::fmt::Arguments;
use wtx::{
  client_api_framework::network::{HttpParams, transport::TransportParams},
  http::ReqBuilder,
};

pub(crate) async fn manage_before_sending(
  api: &mut MelhorEnvio,
  path: Arguments<'_>,
  trans_params: &mut HttpParams,
) -> crate::Result<()> {
  api.common.manage_access_token().await;
  trans_params.ext_req_params_mut().uri.push_path(path)?;
  let headers = &mut trans_params.ext_req_params_mut().headers;
  let _ = ReqBuilder::get(headers).auth_bearer(format_args!("{}", &api.common.access_token))?;
  Ok(())
}
