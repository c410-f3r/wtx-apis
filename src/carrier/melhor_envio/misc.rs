use crate::{carrier::melhor_envio::MelhorEnvio, misc::_apply_auth_header};
use core::fmt::Arguments;
use wtx::client_api_framework::network::{HttpParams, transport::TransportParams};

pub(crate) async fn manage_before_sending(
  api: &mut MelhorEnvio,
  path: Arguments<'_>,
  trans_params: &mut HttpParams,
) -> crate::Result<()> {
  api.common.manage_access_token().await;
  trans_params.ext_req_params_mut().uri.push_path(path)?;
  _apply_auth_header(trans_params, api.common.access_token.as_str())?;
  Ok(())
}
