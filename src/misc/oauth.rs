mod client_credentials;
mod refresh_token;

pub use client_credentials::*;
use core::{fmt::Debug, mem};
pub use refresh_token::*;
use wtx::{
  client_api_framework::{
    Api, SendBytesSource,
    network::{
      HttpParams,
      transport::{SendingReceivingTransport, TransportParams as _},
    },
    pkg::PkgsAux,
  },
  data_transformation::{
    dnsn::{De, DecodeWrapper},
    format::VerbatimResponse,
  },
  http::{Method, Mime},
  misc::{ArrayString, Decode, Vector},
};

pub(crate) type TokenArray = ArrayString<{ 1024 + 512 }>;

/// How the Oauth token should be created.
#[derive(Clone, Copy, Debug, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum OauthGrantType {
  /// Based on redirects with user interaction
  AuthorizationToken,
  /// No user interaction
  ClientCredentials,
  /// Refreshes [`OauthGrantType::AuthorizationToken`] tokens.
  RefreshToken,
}

impl From<OauthGrantType> for &'static str {
  #[inline]
  fn from(from: OauthGrantType) -> Self {
    match from {
      OauthGrantType::AuthorizationToken => "authorization_token",
      OauthGrantType::ClientCredentials => "client_credentials",
      OauthGrantType::RefreshToken => "refresh_token",
    }
  }
}

/// Oauth request
#[derive(Debug, serde::Serialize)]
pub struct OauthRequest<'any> {
  /// Client ID
  pub client_id: &'any str,
  /// Client secret
  pub client_secret: &'any str,
  /// Code
  #[serde(skip_serializing_if = "Option::is_none")]
  pub code: Option<&'any str>,
  /// Code verifier
  #[serde(skip_serializing_if = "Option::is_none")]
  pub code_verifier: Option<&'any str>,
  /// See [`OauthGrantType`].
  pub grant_type: OauthGrantType,
  /// Redirect URI
  #[serde(skip_serializing_if = "Option::is_none")]
  pub redirect_uri: Option<&'any str>,
  /// Refresh token
  #[serde(skip_serializing_if = "Option::is_none")]
  pub refresh_token: Option<&'any str>,
}

/// Oauth response
#[derive(Debug, serde::Deserialize)]
pub struct OauthResponse<T> {
  /// Access token
  pub access_token: T,
  /// Expires in
  pub expires_in: u32,
  /// Refresh token
  pub refresh_token: Option<T>,
  /// Scope
  pub scope: Option<T>,
  /// Token type
  pub token_type: Option<T>,
}

#[inline]
fn encode_req(
  bytes: &mut Vector<u8>,
  (client_id, client_secret, refresh_token): (&str, &str, &str),
  grant_type: OauthGrantType,
  enc_cb: impl FnOnce(&mut Vector<u8>) -> crate::Result<()>,
) -> crate::Result<()> {
  bytes.clear();
  let _ = bytes.extend_from_copyable_slices([
    "grant_type=".as_bytes(),
    <&str>::from(grant_type).as_bytes(),
    "&client_id=".as_bytes(),
    client_id.as_bytes(),
    "&client_secret=".as_bytes(),
    client_secret.as_bytes(),
  ])?;
  if let OauthGrantType::RefreshToken = grant_type {
    let slices = ["&refresh_token=".as_bytes(), refresh_token.as_bytes()];
    let _ = bytes.extend_from_copyable_slices(slices)?;
  }
  enc_cb(bytes)?;
  Ok(())
}

#[inline]
async fn send_req<'de, A, DRSR, T>(
  (api, drsr, mut trans, trans_params): (&mut A, &mut DRSR, T, &mut HttpParams),
  bytes: &'de mut Vector<u8>,
) -> Result<VerbatimResponse<OauthResponse<&'de str>>, A::Error>
where
  A: Api,
  for<'any> T: SendingReceivingTransport<&'any mut HttpParams>,
  for<'any> VerbatimResponse<OauthResponse<&'any str>>: Decode<'any, De<DRSR>>,
{
  trans_params.reset();
  trans_params.ext_req_params_mut().mime = Some(Mime::ApplicationXWwwFormUrlEncoded);
  trans_params.ext_req_params_mut().method = Method::Post;
  let mut pkgs_aux = PkgsAux::from_minimum(&mut *api, drsr, &mut *trans_params);
  mem::swap(&mut pkgs_aux.byte_buffer, bytes);
  let rslt = trans.send_bytes_recv(SendBytesSource::PkgsAux, &mut pkgs_aux).await;
  mem::swap(&mut pkgs_aux.byte_buffer, bytes);
  rslt?;
  let dw = &mut DecodeWrapper::new(bytes);
  let res = VerbatimResponse::<OauthResponse<&str>>::decode(pkgs_aux.drsr, dw)?;
  Ok(res)
}
