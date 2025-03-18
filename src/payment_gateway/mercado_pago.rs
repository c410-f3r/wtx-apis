//! A gateway that process all payments in the Mercado Libre marketplace. Moreover, the company
//! also provides services to third-parties.
//!
//! <https://www.mercadopago.com.ar>
//!
//! ```rust,no_run
//! # async fn fun() -> wtx_apis::Result<()> {
//! use wtx::{client_api_framework::network::HttpParams, data_transformation::dnsn::SerdeJson};
//! use wtx_apis::payment_gateway::mercado_pago::{MercadoPago, PkgsAux};
//!
//! let mercado_pago = MercadoPago::new("CLIENT_ID".into(), "CLIENT_SECRET".into(), true);
//! let mut pkgs_aux =
//!   PkgsAux::from_minimum(mercado_pago, SerdeJson, HttpParams::from_uri("URL".into()));
//! let _ = pkgs_aux.get_payment().params(1).build();
//! # Ok(()) }
//! ```

mod back_urls;
mod chargeback;
#[cfg(all(test, feature = "_integration-tests"))]
mod integration_tests;
mod item;
mod mercado_pago_response;
mod misc;
mod notification;
mod payer;
mod payment;
mod payment_methods;
mod pkg;
mod preference;
mod preference_response;
mod refund;
mod shipments;

use crate::misc::OauthClientCredentials;
use alloc::string::String;
pub use back_urls::BackUrls;
pub use chargeback::*;
pub use item::Item;
pub use mercado_pago_response::{MercadoPagoError, MercadoPagoResponse};
pub use notification::*;
pub use payer::*;
pub use payment::*;
pub use payment_methods::*;
pub use pkg::*;
pub use preference::*;
pub use preference_response::*;
pub use refund::*;
pub use shipments::*;
use wtx::{
  client_api_framework::Api,
  misc::{Lease, LeaseMut},
};

/// Base url for all requests
pub static PROD_URI: &str = "https://api.mercadopago.com";

#[derive(Debug)]
#[doc = _generic_api_doc!()]
#[wtx_macros::api(error(crate::Error), pkgs_aux(PkgsAux), transport(http))]
pub struct MercadoPago {
  common: OauthClientCredentials,
  is_test: bool,
}

impl MercadoPago {
  /// Creates a new instance with the given crendentials.
  pub fn new(
    client_id: String,
    client_secret: String,
    token_ttl_slack: u16,
    is_test: bool,
  ) -> Self {
    MercadoPago {
      common: OauthClientCredentials::new(client_id, client_secret, token_ttl_slack),
      is_test,
    }
  }
}

impl Api for MercadoPago {
  type Error = crate::Error;
  type Id = MercadoPagoId;

  async fn before_sending(&mut self) -> Result<(), Self::Error> {
    Ok(())
  }
}

impl Lease<OauthClientCredentials> for MercadoPago {
  #[inline]
  fn lease(&self) -> &OauthClientCredentials {
    &self.common
  }
}

impl LeaseMut<OauthClientCredentials> for MercadoPago {
  #[inline]
  fn lease_mut(&mut self) -> &mut OauthClientCredentials {
    &mut self.common
  }
}

wtx::create_packages_aux_wrapper!();
