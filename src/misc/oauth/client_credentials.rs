#![allow(dead_code, reason = "Condition feature activation")]

use crate::misc::{
  OauthGrantType, OauthRequest, OauthResponse,
  oauth::{encode_oauth_req, send_oauth_req},
};
use alloc::string::String;
use core::fmt::{Debug, Formatter};
use wtx::{
  calendar::Instant,
  client_api_framework::{
    Api,
    network::{
      HttpParams,
      transport::{SendingReceivingTransport, TransportParams},
    },
  },
  collection::Vector,
  de::{Decode, format::De, protocol::VerbatimDecoder},
  misc::LeaseMut,
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
  for<'any> VerbatimDecoder<OauthResponse<&'any str>>: Decode<'any, De<DRSR>>,
{
  if api.lease_mut().timer.elapsed()?.as_secs() < api.lease_mut().token_ttl.into() {
    return Ok(());
  }
  encode_oauth_req(
    bytes,
    &OauthRequest {
      client_id: &api.lease().client_id,
      client_secret: &api.lease().client_secret,
      code: None,
      code_verifier: None,
      grant_type: OauthGrantType::ClientCredentials,
      redirect_uri: None,
      refresh_token: None,
    },
    enc_cb,
  )?;
  let res = send_oauth_req((api, drsr, trans, trans_params), bytes).await?;
  let OauthClientCredentials { access_token, token_ttl, token_ttl_slack, .. } = api.lease_mut();
  access_token.clear();
  access_token.push_str(res.access_token);
  *token_ttl = if let Some(elem) = res.expires_in.checked_sub((*token_ttl_slack).into()) {
    elem
  } else {
    res.expires_in
  };
  bytes.clear();
  trans_params.reset();
  Ok(())
}
