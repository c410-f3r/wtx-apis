//! HashiCorp Vault

mod pair_vector;
mod pkg;
mod vault_response;

#[cfg(all(test, feature = "_integration-tests"))]
mod integration_tests;
use alloc::string::String;
use core::fmt::{Debug, Formatter};
pub use pair_vector::PairVector;
pub use pkg::*;
pub use vault_response::*;
use wtx::client_api_framework::Api;

#[doc = _generic_api_doc!()]
#[wtx::api(error(crate::Error), pkgs_aux(PkgsAux), transport(http))]
pub struct Vault {
  token: String,
}

impl Vault {
  /// New instance
  #[inline]
  pub fn new(token: String) -> Self {
    Self { token }
  }
}

impl Api for Vault {
  type Error = crate::Error;
  type Id = VaultId;

  async fn before_sending(&mut self) -> Result<(), Self::Error> {
    Ok(())
  }
}

impl Debug for Vault {
  #[inline]
  fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
    f.debug_struct("Vault").finish()
  }
}

wtx::create_packages_aux_wrapper!();
