#![allow(dead_code, reason = "Condition feature activation")]

use crate::misc::{
  OauthGrantType, OauthRequest, OauthResponse, TokenArray,
  oauth::{encode_oauth_req, send_oauth_req},
};
use alloc::string::String;
use core::{
  fmt::{Debug, Formatter},
  future::poll_fn,
  sync::atomic::{AtomicBool, Ordering},
  task::Poll,
};
use wtx::{
  calendar::{DateTime, Duration, Instant, Utc},
  client_api_framework::network::{HttpParams, transport::SendingReceivingTransport},
  collection::{ArrayString, Vector},
  de::{Decode, format::De, protocol::VerbatimDecoder},
  misc::into_rslt,
  sync::{Arc, AtomicCell, AtomicWaker},
};

/// Common attributes used by APIs that integrate Oauth workflows.
#[derive(Debug)]
pub struct OauthRefreshToken {
  pub(crate) access_token: String,
  pub(crate) sync: Arc<OauthRefreshTokenSync>,
}

impl OauthRefreshToken {
  pub(crate) fn new(client_id: String, client_secret: String, token_ttl_slack: u16) -> Self {
    Self {
      access_token: String::new(),
      sync: Arc::new(OauthRefreshTokenSync {
        access_token: AtomicCell::new(TokenArray::new()),
        client_id,
        client_secret,
        needs_access_token_update: AtomicBool::new(true),
        refresh_token: AtomicCell::new(ArrayString::new()),
        token_ttl: AtomicCell::new(DateTime::MIN),
        token_ttl_slack,
        waker: AtomicWaker::new(),
      }),
    }
  }

  /// Awaits until a valid access token is provided
  pub(crate) async fn manage_access_token(&mut self) {
    let mut needs_local_access_token_update = false;
    poll_fn(|cx| {
      if self.sync.needs_access_token_update.load(Ordering::Relaxed) {
        needs_local_access_token_update = true;
        self.sync.waker.register(cx.waker());
        return Poll::Pending;
      }
      Poll::Ready(())
    })
    .await;
    if needs_local_access_token_update || self.access_token.is_empty() {
      self.access_token.clear();
      self.access_token.push_str(self.sync.access_token.load().as_str());
    }
  }
}

/// Differently from client credentials, it is your responsibility to spin up a task that
/// manages the lifecycle of the access token.
pub struct OauthRefreshTokenSync {
  pub(crate) access_token: AtomicCell<TokenArray>,
  pub(crate) client_id: String,
  pub(crate) client_secret: String,
  pub(crate) needs_access_token_update: AtomicBool,
  pub(crate) refresh_token: AtomicCell<TokenArray>,
  pub(crate) token_ttl: AtomicCell<DateTime<Utc>>,
  pub(crate) token_ttl_slack: u16,
  pub(crate) waker: AtomicWaker,
}

impl OauthRefreshTokenSync {
  /// The contents of the access token.
  #[inline]
  pub fn access_token(&self) -> &AtomicCell<TokenArray> {
    &self.access_token
  }

  /// Returns `true` if the access token expired.
  #[inline]
  pub fn needs_refresh(&self) -> crate::Result<bool> {
    let needs_refresh = Instant::now_date_time(0)? >= self.token_ttl.load();
    if needs_refresh {
      self.needs_access_token_update.store(true, Ordering::Relaxed);
    }
    Ok(needs_refresh)
  }

  /// The contents of the refresh token.
  #[inline]
  pub fn refresh_token(&self) -> &AtomicCell<TokenArray> {
    &self.refresh_token
  }

  /// Makes a request that asks for a new access token using the inner refresh token.
  #[inline]
  pub async fn request_params<DRSR, T>(
    &self,
    (drsr, trans, trans_params): (&mut DRSR, T, &mut HttpParams),
    bytes: &mut Vector<u8>,
  ) -> crate::Result<()>
  where
    for<'any> T: SendingReceivingTransport<&'any mut HttpParams>,
    for<'any> VerbatimDecoder<OauthResponse<&'any str>>: Decode<'any, De<DRSR>>,
  {
    encode_oauth_req(
      bytes,
      &OauthRequest {
        client_id: &self.client_id,
        client_secret: &self.client_secret,
        code: None,
        code_verifier: None,
        grant_type: OauthGrantType::RefreshToken,
        redirect_uri: None,
        refresh_token: Some(&self.refresh_token.load()),
      },
      |_| Ok(()),
    )?;
    let res = send_oauth_req((&mut (), drsr, trans, trans_params), bytes).await?;
    self.do_update_params(
      res.access_token,
      into_rslt(res.refresh_token)?,
      Instant::now_date_time(0)?
        .add(Duration::from_seconds(res.expires_in.into()).map_err(wtx::Error::from)?)
        .map_err(wtx::Error::from)?,
    )?;
    Ok(())
  }

  /// The time where the token will expire with the slack value already applied.
  #[inline]
  pub fn token_ttl(&self) -> DateTime<Utc> {
    self.token_ttl.load()
  }

  /// Should be called when the parameters were retrieved throught other means.
  ///
  /// The nanoseconds of `token_ttl` are stripped away.
  pub fn update_params(
    &self,
    access_token: &str,
    refresh_token: &str,
    token_ttl: DateTime<Utc>,
  ) -> crate::Result<()> {
    self.do_update_params(access_token, refresh_token, token_ttl)
  }

  fn do_update_params(
    &self,
    access_token: &str,
    refresh_token: &str,
    token_ttl: DateTime<Utc>,
  ) -> crate::Result<()> {
    self.token_ttl.store(
      token_ttl
        .sub(Duration::from_seconds(self.token_ttl_slack.into()).map_err(wtx::Error::from)?)
        .map_err(wtx::Error::from)?
        .trunc_to_sec(),
    );
    self.needs_access_token_update.store(false, Ordering::Relaxed);
    self.access_token.store(access_token.try_into()?);
    self.refresh_token.store(refresh_token.try_into()?);
    self.waker.wake();
    Ok(())
  }
}

impl Debug for OauthRefreshTokenSync {
  #[inline]
  fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
    f.debug_struct("OauthRefreshTokenSync").finish()
  }
}
