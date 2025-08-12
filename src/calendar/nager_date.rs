//! Public holidays for more than 100 countries.
//!
//! <https://date.nager.at>
//!
//! ```rust,no_run
//! # async fn fun() -> wtx_apis::Result<()> {
//! use wtx::{client_api_framework::network::HttpParams, de::format::SerdeJson};
//! use wtx_apis::calendar::nager_date::{NagerDate, PkgsAux};
//!
//! let mut pkgs_aux =
//!   PkgsAux::from_minimum(NagerDate, SerdeJson, HttpParams::from_uri("URL".into()));
//! let _ = pkgs_aux.v3_country_info().params("es").build();
//! # Ok(()) }
//! ```

#[cfg(all(test, feature = "_integration-tests"))]
mod integration_tests;
mod pkg;

pub use pkg::*;

#[derive(Debug)]
#[doc = _generic_api_doc!()]
#[wtx::api(error(crate::Error), mode(auto), pkgs_aux(PkgsAux), transport(http))]
pub struct NagerDate;

wtx::create_packages_aux_wrapper!();
