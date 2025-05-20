//! A freight platform that connects carriers and store to speed-up e-commerce delivers.

mod calculate_shipment_request;
mod calculate_shipment_response;
mod insert_cart_request;
mod insert_cart_response;
#[cfg(all(test, feature = "_integration-tests"))]
mod integration_tests;
mod melhor_envio_result;
mod misc;
mod pkg;

use crate::misc::{OauthRefreshToken, OauthRefreshTokenSync};
use alloc::string::String;
pub use calculate_shipment_request::*;
pub use calculate_shipment_response::*;
use core::{fmt::Debug, time::Duration};
pub use insert_cart_request::*;
pub use insert_cart_response::*;
pub use melhor_envio_result::*;
pub use pkg::*;
use wtx::{
  misc::{Lease, LeaseMut},
  sync::Arc,
};

/// Development URI
pub static DEV_URI: &str = "https://sandbox.melhorenvio.com.br";
/// Production URI
pub static PROD_URI: &str = "https://www.melhorenvio.com.br";

#[doc = _generic_api_doc!()]
#[wtx_macros::api(error(crate::Error), mode(auto), pkgs_aux(PkgsAux), transport(http))]
#[derive(Debug)]
pub struct MelhorEnvio {
  common: OauthRefreshToken,
}

impl MelhorEnvio {
  /// New instance
  #[inline]
  pub fn new(client_id: String, client_secret: String, token_ttl_slack: u16) -> Self {
    const _1_MIN: Duration = Duration::from_secs(30);
    Self { common: OauthRefreshToken::new(client_id, client_secret, token_ttl_slack) }
  }

  /// Manages tokens in concurrent scenarios.
  pub fn sync(&self) -> &Arc<OauthRefreshTokenSync> {
    &self.common.sync
  }
}

impl Lease<OauthRefreshToken> for MelhorEnvio {
  #[inline]
  fn lease(&self) -> &OauthRefreshToken {
    &self.common
  }
}

impl LeaseMut<OauthRefreshToken> for MelhorEnvio {
  #[inline]
  fn lease_mut(&mut self) -> &mut OauthRefreshToken {
    &mut self.common
  }
}

wtx::create_packages_aux_wrapper!();
