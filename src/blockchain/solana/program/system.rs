//! Fundamental system elements

/// Accounts required to execute instructions
pub mod accounts;
/// Data stored in the blockchain
pub mod data;
/// Program instructions
pub mod instructions;

use crate::{blockchain::solana::SolanaAddressHashStr, misc::HashArray32Static};

/// Data related to the transfer instruction.
#[derive(Debug, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TransferInstruction {
  /// Receiving Base58 identifier
  pub destination: SolanaAddressHashStr,
  /// Transferred lamports
  pub lamports: u64,
  /// Sending Base58 identifier
  pub source: SolanaAddressHashStr,
}

/// Public address of the program
pub const ID: HashArray32Static = HashArray32Static::new(
  [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
  "11111111111111111111111111111111",
);
