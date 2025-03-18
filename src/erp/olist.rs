//! A brazilian ERP.
//!
//! <https://olist.com/sistema-erp/>

mod associated_entity;
mod civil_status;
mod contact_post;
mod contributor;
mod freight_responsibility;
mod gender;
mod info;
#[cfg(all(test, feature = "_integration-tests"))]
mod integration_tests;
mod misc;
mod olist_result;
mod order;
mod order_get;
mod order_post;
mod payment_method;
mod person_ty;
mod pkg;
mod plan;
mod product_webhook;
mod shipment_method;
mod tributary_regime;
mod webhook_product_mapping;
mod webhook_request;

use crate::misc::{OauthRefreshToken, OauthRefreshTokenSync};
use alloc::string::String;
pub use associated_entity::*;
pub use civil_status::CivilStatus;
pub use contact_post::*;
pub use contributor::Contributor;
use core::{
  fmt::{Debug, Formatter},
  time::Duration,
};
pub use freight_responsibility::FreightResponsibility;
pub use gender::Gender;
pub use info::*;
pub use olist_result::*;
pub use order::*;
pub use order_get::*;
pub use order_post::*;
pub use payment_method::*;
pub use person_ty::PersonTy;
pub use pkg::*;
pub use plan::Plan;
pub use product_webhook::*;
pub use shipment_method::*;
pub use tributary_regime::TributaryRegime;
pub use webhook_product_mapping::WebhookProductMapping;
pub use webhook_request::WebhookRequest;
use wtx::{
  client_api_framework::{
    Api,
    misc::{RequestLimit, RequestThrottling},
  },
  misc::{Arc, Lease, LeaseMut},
};

/// Base URI
pub const API_PROD_URI: &str = "https://api.tiny.com.br/public-api/v3";
/// Oauth API
pub const ACC_PROD_URI: &str =
  "https://accounts.tiny.com.br/realms/tiny/protocol/openid-connect/token";

#[doc = _generic_api_doc!()]
#[wtx_macros::api(error(crate::Error), pkgs_aux(PkgsAux), transport(http))]
pub struct Olist {
  common: OauthRefreshToken,
  rt: RequestThrottling,
}

impl Olist {
  /// New instance
  #[inline]
  pub fn new(
    client_id: String,
    client_secret: String,
    token_ttl_slack: u16,
    plan: Plan,
    refresh_token: &str,
  ) -> crate::Result<Self> {
    const _1_MIN: Duration = Duration::from_secs(30);
    Ok(Self {
      common: OauthRefreshToken::new(client_id, client_secret, token_ttl_slack, refresh_token)?,
      rt: RequestThrottling::from_rl(match plan {
        Plan::Crescer => RequestLimit::new(30, _1_MIN),
        Plan::Evoluir => RequestLimit::new(60, _1_MIN),
        Plan::Potencializar => RequestLimit::new(100, _1_MIN),
      }),
    })
  }

  /// Manages tokens in concurrent scenarios.
  pub fn sync(&self) -> &Arc<OauthRefreshTokenSync> {
    &self.common.sync
  }
}

impl Api for Olist {
  type Error = crate::Error;
  type Id = OlistId;

  async fn before_sending(&mut self) -> Result<(), Self::Error> {
    self.rt.rc.update_params(&self.rt.rl).await?;
    Ok(())
  }
}

impl Lease<OauthRefreshToken> for Olist {
  #[inline]
  fn lease(&self) -> &OauthRefreshToken {
    &self.common
  }
}

impl LeaseMut<OauthRefreshToken> for Olist {
  #[inline]
  fn lease_mut(&mut self) -> &mut OauthRefreshToken {
    &mut self.common
  }
}

impl Debug for Olist {
  #[inline]
  fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
    f.debug_struct("Olist").finish()
  }
}

wtx::create_packages_aux_wrapper!();
