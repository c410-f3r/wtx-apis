//! Offers various delivery services for e-commerce fulfillment.

mod cancel_order_req;
mod cancel_order_res;
#[cfg(all(test, feature = "_integration-tests"))]
mod integration_tests;
mod misc;
mod packge_format;
mod pkg;
mod quote_freight_req;
mod quote_freight_res;
mod send_freight_req;
mod send_freight_res;
mod super_frete_response;

use alloc::string::String;
pub use cancel_order_req::*;
pub use cancel_order_res::*;
use core::fmt::{Debug, Formatter};
pub use packge_format::PackageFormat;
pub use pkg::*;
pub use quote_freight_req::*;
pub use quote_freight_res::*;
pub use send_freight_req::*;
pub use send_freight_res::*;
pub use super_frete_response::*;

/// Development URI
pub static DEV_URI: &str = "https://sandbox.superfrete.com";
/// Production URI
pub static PROD_URI: &str = "https://api.superfrete.com";

#[doc = _generic_api_doc!()]
#[wtx_macros::api(error(crate::Error), mode(auto), pkgs_aux(PkgsAux), transport(http))]
pub struct SuperFrete {
  token: String,
}

impl SuperFrete {
  /// Creates a new instance with the given token.
  pub fn new(token: String) -> Self {
    SuperFrete { token }
  }
}

impl Debug for SuperFrete {
  #[inline]
  fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
    f.debug_struct("SuperFrete").finish()
  }
}

wtx::create_packages_aux_wrapper!();
