//! Fake data for testing and prototyping.
//!
//! <http://jsonplaceholder.typicode.com>
//!
//! ```rust,no_run
//! # async fn fun() -> wtx_apis::Result<()> {
//! use wtx::{
//!   dnsn::SerdeJson,
//!   network::{HttpMethod, HttpParams},
//! };
//! use wtx_apis::{
//!   misc::PkgsAux,
//!   test_data::json_placeholder::{GenericParams, JsonPlaceholder},
//! };
//!
//! let mut pkgs_aux =
//!   PkgsAux::from_minimum(JsonPlaceholder, SerdeJson, (HttpParams::from_url("URL")?));
//! let _ = pkgs_aux.albums().params(GenericParams::new(None, HttpMethod::Get, None, &[])).build();
//! # Ok(()) }
//! ```

#[cfg(all(test, feature = "_integration-tests"))]
mod integration_tests;
mod pkg;

pub use pkg::*;
use wtx::client_api_framework::Api;

#[derive(Debug)]
#[doc = _generic_api_doc!()]
#[wtx_macros::api_types(pkgs_aux(crate::misc::PkgsAux), transport(http))]
pub struct JsonPlaceholder;

impl Api for JsonPlaceholder {
  type Error = crate::Error;
}
