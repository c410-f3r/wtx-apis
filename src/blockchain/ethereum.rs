//! Ethereum is a decentralized, open-source blockchain with smart contract functionality.
//!
//! <https://web3js.readthedocs.io/>
//!
//! Built upon the logic constructed in <https://github.com/tomusdrw/rust-web3>.
//!
//! ```rust,no_run
//! # async fn fun() -> wtx_apis::Result<()> {
//! use wtx::client_api_framework::{dnsn::SerdeJson, network::HttpParams};
//! use wtx_apis::blockchain::ethereum::{Ethereum, PkgsAux};
//!
//! let mut pkgs_aux =
//!   PkgsAux::from_minimum(Ethereum::new(None), SerdeJson, HttpParams::from_uri("URL"));
//! let _ = pkgs_aux.eth_block_number().build();
//! # Ok(()) }
//! ```

wtx::create_packages_aux_wrapper!();

mod access_list;
mod access_list_item;
mod block_id;
mod block_number;
mod call_request;
pub mod contract;
#[cfg(all(test, feature = "_integration-tests"))]
mod integration_tests;
mod pkg;
mod raw_transaction;
mod receipt;
mod transaction;
mod transaction_condition;
mod transaction_request;
mod types;

pub use access_list::AccessList;
pub use access_list_item::AccessListItem;
pub use block_id::BlockId;
pub use block_number::BlockNumber;
pub use call_request::CallRequest;
pub use pkg::*;
pub use raw_transaction::RawTransaction;
pub use receipt::Receipt;
pub use transaction::Transaction;
pub use transaction_condition::TransactionCondition;
pub use transaction_request::TransactionRequest;
pub use types::*;
use wtx::client_api_framework::{misc::RequestThrottling, Api};

#[derive(Debug)]
#[doc = _generic_api_doc!()]
#[wtx_macros::api_params(pkgs_aux(PkgsAux), transport(http, ws))]
pub struct Ethereum {
  /// If some, tells that each request must respect calling intervals.
  pub rt: Option<RequestThrottling>,
}

impl Ethereum {
  /// If desired, it is possible to instantiate directly instead of using this method.
  pub const fn new(rt: Option<RequestThrottling>) -> Self {
    Self { rt }
  }
}

impl Api for Ethereum {
  type Error = crate::Error;

  async fn before_sending(&mut self) -> Result<(), Self::Error> {
    if let Some(ref mut rt) = self.rt {
      rt.rc.update_params(&rt.rl).await?;
    }
    Ok(())
  }
}
