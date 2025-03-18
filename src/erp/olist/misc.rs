use crate::{erp::olist::Olist, misc::_apply_auth_header};
use wtx::{
  client_api_framework::network::{HttpParams, transport::TransportParams},
  misc::UriString,
};

pub(crate) async fn manage_before_sending(
  api: &mut Olist,
  trans_params: &mut HttpParams,
  cb: impl FnOnce(&mut UriString) -> wtx::Result<()>,
) -> crate::Result<()> {
  api.common.manage_access_token().await;
  cb(&mut trans_params.ext_req_params_mut().uri)?;
  _apply_auth_header(trans_params, &api.common.access_token)?;
  Ok(())
}
