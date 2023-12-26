//! Collection of APIs based on the wtx framework.
//!
//! Most of the API structures are markers used to guide different type implementations.

#![cfg_attr(not(feature = "std"), no_std)]

extern crate alloc;

#[macro_use]
mod macros;

pub mod blockchain;
pub mod calendar;
mod error;
pub mod misc;
pub mod payment_gateway;
pub mod series;
pub mod test_data;

pub use error::Error;

/// Alias of `core::result::Result<T, wtx_apis::Error>`
pub type Result<T> = core::result::Result<T, Error>;
