//! GraphQL API based on the television show Rick and Morty.
//!
//! <https://github.com/afuh/rick-and-morty-api>
//!
//! ```rust,no_run
//! # async fn fun() -> wtx_apis::Result<()> {
//! use wtx::{client_api_framework::network::HttpParams, data_transformation::dnsn::SerdeJson};
//! use wtx_apis::series::rick_and_morty::{PkgsAux, RickAndMorty};
//!
//! let mut pkgs_aux =
//!   PkgsAux::from_minimum(RickAndMorty, SerdeJson, HttpParams::from_uri("URL".into()));
//! let _ = pkgs_aux.character().data(&mut String::new(), 1)?.build();
//! # Ok(()) }
//! ```
//!
//! #### Noteworthy
//!
//! The architecture here is purposely "REST-ish" to avoid third-party dependencies and quickly
//! exercise the actual format expected by GraphQL requests and responses for `wtx`.

#[cfg(all(test, feature = "_integration-tests"))]
mod integration_tests;
mod pkg;

wtx::create_packages_aux_wrapper!();

pub use pkg::*;
use wtx::client_api_framework::Api;

#[derive(Debug)]
#[doc = _generic_api_doc!()]
#[wtx_macros::api_params(pkgs_aux(PkgsAux), transport(http))]
pub struct RickAndMorty;

impl Api for RickAndMorty {
  type Error = crate::Error;
}
