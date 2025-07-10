//! Fake data for testing and prototyping.
//!
//! <https://jsonplaceholder.typicode.com>
//!
//! ```rust,no_run
//! # async fn fun() -> wtx_apis::Result<()> {
//! use wtx::{client_api_framework::network::HttpParams, de::dnsn::SerdeJson, http::Method};
//! use wtx_apis::test_data::json_placeholder::{GenericParams, JsonPlaceholder, PkgsAux};
//!
//! let mut pkgs_aux =
//!   PkgsAux::from_minimum(JsonPlaceholder, SerdeJson, (HttpParams::from_uri("URL".into())));
//! let _ = pkgs_aux.albums().params(GenericParams::new(None, Method::Get, None, &[])).build();
//! # Ok(()) }
//! ```

#[cfg(all(test, feature = "_integration-tests"))]
mod integration_tests;
mod pkg;

pub use pkg::*;

#[derive(Debug)]
#[doc = _generic_api_doc!()]
#[wtx::api(error(crate::Error), mode(auto), pkgs_aux(PkgsAux), transport(http))]
pub struct JsonPlaceholder;

wtx::create_packages_aux_wrapper!();
