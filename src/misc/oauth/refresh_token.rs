#![allow(dead_code, reason = "Condition feature activation")]

use alloc::string::String;
use core::{
  fmt::{Debug, Formatter},
  future::poll_fn,
  sync::atomic::{AtomicBool, AtomicU32, Ordering},
  task::Poll,
  time::Duration,
};
use wtx::{
  client_api_framework::network::{HttpParams, transport::SendingReceivingTransport},
  data_transformation::{dnsn::De, format::VerbatimResponse},
  misc::{Arc, AtomicCell, AtomicWaker, Decode, GenericTime, Vector, into_rslt},
};

use crate::misc::{
  OauthGrantType, OauthResponse, TokenArray,
  oauth::{encode_req, send_req},
};

/// Common attributes used by APIs that integrate Oauth workflows.
#[derive(Debug)]
pub struct OauthRefreshToken {
  pub(crate) access_token: String,
  pub(crate) sync: Arc<OauthRefreshTokenSync>,
}

impl OauthRefreshToken {
  pub(crate) fn new(
    client_id: String,
    client_secret: String,
    token_ttl_slack: u16,
    refresh_token: &str,
  ) -> crate::Result<Self> {
    Ok(Self {
      access_token: String::new(),
      sync: Arc::new(OauthRefreshTokenSync {
        access_token: AtomicCell::new(TokenArray::new()),
        client_id,
        client_secret,
        needs_access_token_update: AtomicBool::new(true),
        refresh_token: AtomicCell::new(refresh_token.try_into()?),
        timer: AtomicCell::new(GenericTime::now()),
        token_ttl: AtomicU32::new(0),
        token_ttl_slack,
        waker: AtomicWaker::new(),
      }),
    })
  }

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
  pub(crate) timer: AtomicCell<GenericTime>,
  pub(crate) token_ttl: AtomicU32,
  pub(crate) token_ttl_slack: u16,
  pub(crate) waker: AtomicWaker,
}

impl OauthRefreshTokenSync {
  /// The contents of the access token.
  #[inline]
  pub fn access_token(&self) -> &AtomicCell<TokenArray> {
    &self.access_token
  }

  /// Returns `true` if the TTL of a token was expired.
  ///
  /// The actual TTL is calculated as the returned API's TTL minus the user-provided TTL slack.
  #[inline]
  pub fn needs_refresh(&self) -> crate::Result<bool> {
    let elapsed = self.timer.load().elapsed()?.as_secs();
    let needs_refresh = elapsed >= self.token_ttl.load(Ordering::Relaxed).into();
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
  pub async fn request_tokens<DRSR, T>(
    &self,
    (drsr, trans, trans_params): (&mut DRSR, T, &mut HttpParams),
    bytes: &mut Vector<u8>,
  ) -> crate::Result<()>
  where
    for<'any> T: SendingReceivingTransport<&'any mut HttpParams>,
    for<'any> VerbatimResponse<OauthResponse<&'any str>>: Decode<'any, De<DRSR>>,
  {
    encode_req(
      bytes,
      (&self.client_id, &self.client_secret, &self.refresh_token.load()),
      OauthGrantType::RefreshToken,
      |_| Ok(()),
    )?;
    let res = send_req((&mut (), drsr, trans, trans_params), bytes).await?;
    update_access_token(&self.access_token, res.data.access_token)?;
    update_token_ttl(&self.token_ttl, res.data.expires_in, self.token_ttl_slack);
    self.needs_access_token_update.store(false, Ordering::Relaxed);
    self.refresh_token.store(into_rslt(res.data.refresh_token)?.try_into()?);
    self.timer.store(GenericTime::now());
    self.waker.wake();
    Ok(())
  }

  /// The time where the token will expire.
  #[inline]
  pub fn token_expiration(&self) -> crate::Result<GenericTime> {
    Ok(
      self
        .timer
        .load()
        .checked_add(Duration::from_secs(self.token_ttl.load(Ordering::Relaxed).into()))?,
    )
  }
}

impl Debug for OauthRefreshTokenSync {
  #[inline]
  fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
    f.debug_struct("OauthRefreshTokenSync").finish()
  }
}

#[inline]
fn update_access_token(
  access_token: &AtomicCell<TokenArray>,
  access_token_new: &str,
) -> crate::Result<()> {
  access_token.store(TokenArray::try_from(access_token_new)?);
  Ok(())
}

#[inline]
fn update_token_ttl(token_ttl: &AtomicU32, token_ttl_new: u32, token_ttl_slack: u16) {
  token_ttl.store(token_ttl_new.saturating_sub(token_ttl_slack.into()), Ordering::Relaxed);
}
