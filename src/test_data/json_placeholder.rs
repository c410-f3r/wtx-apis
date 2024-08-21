//! Fake data for testing and prototyping.
//!
//! <https://jsonplaceholder.typicode.com>
//!
//! ```rust,no_run
//! # async fn fun() -> wtx_apis::Result<()> {
//! use wtx::{
//!   client_api_framework::network::HttpParams, data_transformation::dnsn::SerdeJson, http::Method,
//! };
//! use wtx_apis::test_data::json_placeholder::{GenericParams, JsonPlaceholder, PkgsAux};
//!
//! let mut pkgs_aux =
//!   PkgsAux::from_minimum(JsonPlaceholder, SerdeJson, (HttpParams::from_uri("URL")));
//! let _ = pkgs_aux.albums().params(GenericParams::new(None, Method::Get, None, &[])).build();
//! # Ok(()) }
//! ```

#[cfg(all(test, feature = "_integration-tests"))]
mod integration_tests;
mod pkg;

wtx::create_packages_aux_wrapper!();

pub use pkg::*;
use wtx::client_api_framework::Api;

#[derive(Debug)]
#[doc = _generic_api_doc!()]
#[wtx_macros::api_params(pkgs_aux(PkgsAux), transport(http))]
pub struct JsonPlaceholder;

impl Api for JsonPlaceholder {
  type Error = crate::Error;
}
