//! Collection of APIs based on the wtx framework.
//!
//! Most of the API structures are markers used to guide different type implementations.

#![no_std]

extern crate alloc;
#[cfg(feature = "std")]
extern crate std;

#[macro_use]
mod macros;

pub mod blockchain;
pub mod carrier;
pub mod erp;
mod error;
pub mod exchange;
pub mod misc;
pub mod payment_gateway;
pub mod secret_management;
#[cfg(all(feature = "std", test))]
mod tests;
mod tuple_impls;

pub use error::Error;
/// Alias of `core::result::Result<T, wtx_apis::Error>`
pub type Result<T> = core::result::Result<T, Error>;

/// For example, a token, a stock or a commodity.
pub type AssetName = wtx::collection::ArrayStringU8<10>;
/// For example, BTC/USDT or KCH26/USD
pub type PairName = wtx::collection::ArrayStringU8<14>;
