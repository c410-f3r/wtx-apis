//! Ethereum is a decentralized, open-source blockchain with smart contract functionality.
//!
//! <https://web3js.readthedocs.io/>
//!
//! Built upon the logic constructed in <https://github.com/tomusdrw/rust-web3>.
//!
//! ```rust,no_run
//! # async fn fun() -> wtx_apis::Result<()> {
//! use wtx::{dnsn::SerdeJson, network::HttpParams};
//! use wtx_apis::{blockchain::ethereum::Ethereum, misc::PkgsAux};
//!
//! let mut pkgs_aux = PkgsAux::from_minimum(Ethereum, SerdeJson, HttpParams::from_url("URL")?);
//! let _ = pkgs_aux.eth_block_number().build();
//! # Ok(()) }
//! ```

mod access_list;
mod access_list_item;
mod block_id;
mod block_number;
mod call_request;
//mod contract;
#[cfg(all(test, feature = "_integration-tests"))]
mod integration_tests;
mod pkg;
mod raw_transaction;
mod receipt;
mod transaction;
mod transaction_condition;
mod transaction_request;
mod types;

pub use access_list::*;
pub use access_list_item::*;
pub use block_id::*;
pub use block_number::*;
pub use call_request::*;
//pub use contract::Contract;
pub use pkg::*;
pub use raw_transaction::*;
pub use receipt::*;
pub use transaction::*;
pub use transaction_condition::*;
pub use transaction_request::*;
pub use types::*;
use wtx::client_api_framework::Api;

#[derive(Debug)]
#[doc = _generic_api_doc!()]
#[wtx_macros::api_types(pkgs_aux(crate::misc::PkgsAux), transport(http, ws))]
pub struct Ethereum;

impl Api for Ethereum {
  type Error = crate::Error;
}
