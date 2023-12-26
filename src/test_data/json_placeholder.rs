//! Fake data for testing and prototyping.
//!
//! <http://jsonplaceholder.typicode.com>
//!
//! ```rust,no_run
//! # async fn fun() -> wtx_apis::Result<()> {
//! use wtx::client_api_framework::{
//!   dnsn::SerdeJson,
//!   network::{HttpMethod, HttpParams},
//! };
//! use wtx_apis::test_data::json_placeholder::{GenericParams, JsonPlaceholder, PkgsAux};
//!
//! let mut pkgs_aux =
//!   PkgsAux::from_minimum(JsonPlaceholder, SerdeJson, (HttpParams::from_uri("URL")));
//! let _ = pkgs_aux.albums().params(GenericParams::new(None, HttpMethod::Get, None, &[])).build();
//! # Ok(()) }
//! ```

wtx::create_packages_aux_wrapper!();

#[cfg(all(test, feature = "_integration-tests"))]
mod integration_tests;
mod pkg;

pub use pkg::*;
use wtx::client_api_framework::Api;

#[derive(Debug)]
#[doc = _generic_api_doc!()]
#[wtx_macros::api_types(pkgs_aux(PkgsAux), transport(http))]
pub struct JsonPlaceholder;

impl Api for JsonPlaceholder {
  type Error = crate::Error;
}
