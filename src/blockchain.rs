//! Growing list of records, called blocks, that are securely linked together using cryptography.

mod confirm_transaction_params;
#[cfg(feature = "ethereum")]
pub mod ethereum;
#[cfg(feature = "solana")]
pub mod solana;

pub use confirm_transaction_params::*;
use wtx::collection::ArrayStringU8;

/// For example, BTC or ETH.
pub type AssetName = ArrayStringU8<5>;
