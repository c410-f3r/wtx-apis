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
  collection::{ArrayStringU16, IndexedStorageMut, Vector},
  de::{
    Decode,
    format::{De, DecodeWrapper},
    protocol::VerbatimDecoder,
  },
  http::{Method, Mime},
};

pub(crate) type TokenArray = ArrayStringU16<{ 2048 + 256 }>;

/// How the Oauth token should be created.
#[derive(Clone, Copy, Debug, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum OauthGrantType {
  /// Based on redirects with user interaction
  AuthorizationCode,
  /// No user interaction
  ClientCredentials,
  /// Refreshes [`OauthGrantType::AuthorizationCode`] tokens.
  RefreshToken,
}

impl From<OauthGrantType> for &'static str {
  #[inline]
  fn from(from: OauthGrantType) -> Self {
    match from {
      OauthGrantType::AuthorizationCode => "authorization_code",
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

/// Should be called before invoking [`send_oauth_req`].
#[inline]
pub fn encode_oauth_req(
  bytes: &mut Vector<u8>,
  req: &OauthRequest<'_>,
  enc_cb: impl FnOnce(&mut Vector<u8>) -> crate::Result<()>,
) -> crate::Result<()> {
  bytes.clear();
  let _ = bytes.extend_from_copyable_slices([
    "grant_type=".as_bytes(),
    <&str>::from(req.grant_type).as_bytes(),
    "&client_id=".as_bytes(),
    req.client_id.as_bytes(),
    "&client_secret=".as_bytes(),
    req.client_secret.as_bytes(),
  ])?;
  if let Some(elem) = req.code {
    let slices = ["&code=".as_bytes(), elem.as_bytes()];
    let _ = bytes.extend_from_copyable_slices(slices)?;
  }
  if let Some(elem) = req.redirect_uri {
    let slices = ["&redirect_uri=".as_bytes(), elem.as_bytes()];
    let _ = bytes.extend_from_copyable_slices(slices)?;
  }
  if let Some(elem) = req.refresh_token {
    let slices = ["&refresh_token=".as_bytes(), elem.as_bytes()];
    let _ = bytes.extend_from_copyable_slices(slices)?;
  }
  enc_cb(bytes)?;
  Ok(())
}

/// Sends a [`OauthRequest`] that should first be encoded with [`encode_oauth_req`].
#[inline]
pub async fn send_oauth_req<'de, A, DRSR, T>(
  (api, drsr, mut trans, trans_params): (&mut A, &mut DRSR, T, &mut HttpParams),
  bytes: &'de mut Vector<u8>,
) -> Result<OauthResponse<&'de str>, A::Error>
where
  A: Api,
  for<'any> T: SendingReceivingTransport<&'any mut HttpParams>,
  for<'any> VerbatimDecoder<OauthResponse<&'any str>>: Decode<'any, De<DRSR>>,
{
  trans_params.ext_req_params_mut().headers.clear();
  trans_params.ext_req_params_mut().method = Method::Post;
  trans_params.ext_req_params_mut().mime = Some(Mime::ApplicationXWwwFormUrlEncoded);
  let mut pkgs_aux = PkgsAux::from_minimum(&mut *api, drsr, &mut *trans_params);
  mem::swap(&mut pkgs_aux.byte_buffer, bytes);
  pkgs_aux.log_body();
  let rslt = trans.send_bytes_recv(SendBytesSource::PkgsAux, &mut pkgs_aux).await;
  mem::swap(&mut pkgs_aux.byte_buffer, bytes);
  rslt?;
  let dw = &mut DecodeWrapper::new(bytes);
  let res = VerbatimDecoder::<OauthResponse<&str>>::decode(pkgs_aux.drsr, dw)?;
  Ok(res.data)
}
