mod builder_info;
#[cfg(all(test, feature = "_integration-tests"))]
mod integration_tests;
mod misc;
mod order;
mod payload;
mod pkg;

pub use builder_info::BuilderInfo;
pub use payload::{ExchangePayload, ExchangeResponse};
use wtx::client_api_framework::Api;

#[derive(Debug)]
#[doc = _generic_api_doc!()]
#[wtx::api(error(crate::Error), pkgs_aux(PkgsAux), transport(http))]
pub struct Hyperliquid {}

impl Hyperliquid {
  /// Creates a new instance with the given credentials.
  pub const fn new() -> Self {
    Self {}
  }
}

impl Api for Hyperliquid {
  type Error = crate::Error;
  type Id = HyperliquidId;

  async fn before_sending(&mut self) -> Result<(), Self::Error> {
    Ok(())
  }
}

wtx::create_packages_aux_wrapper!();
