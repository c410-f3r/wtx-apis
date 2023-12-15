use crate::{blockchain::solana::SolanaAddressHashStr, misc::MaxNumberStr};

/// Data related to the transfer instruction.
#[derive(Debug, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TransferInstruction {
  /// Transferred lamports
  pub amount: MaxNumberStr,
  /// Authority
  pub authority: SolanaAddressHashStr,
  /// Receiving Base58 identifier
  pub destination: SolanaAddressHashStr,
  /// Sending Base58 identifier
  pub source: SolanaAddressHashStr,
}
