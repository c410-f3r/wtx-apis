#![allow(dead_code, reason = "Condition feature activation")]

use crate::misc::{
  OauthGrantType, OauthResponse,
  oauth::{encode_req, send_req},
};
use alloc::string::String;
use core::fmt::{Debug, Formatter};
use wtx::{
  client_api_framework::{
    Api,
    network::{HttpParams, transport::SendingReceivingTransport},
  },
  collection::Vector,
  data_transformation::{dnsn::De, format::VerbatimResponse},
  misc::{Decode, LeaseMut},
  time::Instant,
};

/// Common attributes used by APIs that integrate Oauth workflows.
pub struct OauthClientCredentials {
  pub(crate) access_token: String,
  pub(crate) client_id: String,
  pub(crate) client_secret: String,
  pub(crate) timer: Instant,
  pub(crate) token_ttl_slack: u16,
  pub(crate) token_ttl: u32,
}

impl OauthClientCredentials {
  pub(crate) fn new(client_id: String, client_secret: String, token_ttl_slack: u16) -> Self {
    Self {
      access_token: String::new(),
      client_id,
      client_secret,
      timer: Instant::now(),
      token_ttl_slack,
      token_ttl: 0,
    }
  }
}

impl Debug for OauthClientCredentials {
  #[inline]
  fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
    f.debug_struct("OauthClientCredentials").finish()
  }
}

#[inline]
pub(crate) async fn _manage_client_credentials<A, DRSR, T>(
  (api, drsr, trans, trans_params): (&mut A, &mut DRSR, T, &mut HttpParams),
  bytes: &mut Vector<u8>,
  enc_cb: impl FnOnce(&mut Vector<u8>) -> crate::Result<()>,
) -> crate::Result<()>
where
  A: Api<Error = crate::Error> + LeaseMut<OauthClientCredentials>,
  for<'any> T: SendingReceivingTransport<&'any mut HttpParams>,
  for<'any> VerbatimResponse<OauthResponse<&'any str>>: Decode<'any, De<DRSR>>,
{
  if api.lease_mut().timer.elapsed()?.as_secs() < api.lease_mut().token_ttl.into() {
    return Ok(());
  }
  encode_req(
    bytes,
    (&api.lease().client_id, &api.lease().client_secret, ""),
    OauthGrantType::ClientCredentials,
    enc_cb,
  )?;
  let res = send_req((api, drsr, trans, trans_params), bytes).await?;
  let OauthClientCredentials { access_token, token_ttl, token_ttl_slack, .. } = api.lease_mut();
  access_token.clear();
  access_token.push_str(res.data.access_token);
  *token_ttl = if let Some(elem) = res.data.expires_in.checked_sub((*token_ttl_slack).into()) {
    elem
  } else {
    res.data.expires_in
  };
  Ok(())
}
