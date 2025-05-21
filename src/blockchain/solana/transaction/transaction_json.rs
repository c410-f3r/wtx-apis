use crate::blockchain::solana::{
  SolanaAddressHashStr, SolanaBlockhashStr, SolanaProgramName, SolanaSignatureHashStr,
  program::{
    spl_token::{self, TransferCheckedInstruction},
    system,
  },
};
use alloc::string::String;
use wtx::collection::{ArrayString, Vector};

/// A json instruction can be expressed in different formats.
#[allow(
  // Only used in tx deserialization that is already boxed
  variant_size_differences
)]
#[derive(Debug, serde::Deserialize)]
#[serde(rename_all = "camelCase", untagged)]
pub enum InstructionJsonGeneric {
  /// Compiled
  Compiled(CompiledInstructionJson),
  /// Legacy name that actually means different sets of JSON-based instructions
  Parsed(InstructionJsonParsedGeneric),
}

/// A parsed json instruction can be expressed in even more different formats
#[derive(Debug, serde::Deserialize)]
#[serde(rename_all = "camelCase", untagged)]
pub enum InstructionJsonParsedGeneric {
  /// Partially decoded
  PartiallyDecoded(InstructionPartiallyDecoded),
  /// Parsed
  Parsed(InstructionJsonParsedOverall),
}

/// Contains known instructions that can be represented.
#[derive(Debug, serde::Deserialize)]
#[serde(rename_all = "camelCase", untagged)]
pub enum InstructionJsonParsedInfo {
  /// Spl-token transfer
  SplTokenTransferInstruction(spl_token::TransferInstruction),
  /// Spl-token checked transfer
  SplTokenTransferCheckedInstruction(TransferCheckedInstruction),
  /// Spl-token transfer
  SystemTransferInstruction(system::TransferInstruction),
  /// Unsupported
  #[serde(deserialize_with = "crate::misc::deserialize_ignore_any")]
  Unknown,
}

/// Json data expressed as raw bytes.
#[derive(Debug, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CompiledInstructionJson {
  /// Index in regards to the block array of programs.
  pub program_id_index: u8,
  /// Indexes in regards to the block array of accounts.
  pub accounts: Vector<u8>,
  /// Opaque data
  pub data: String,
}

/// Decoded instruction contained in other outer instructions.
#[derive(Debug, serde::Deserialize)]
pub struct InnerInstructionJson {
  /// Index in regards to the block array of instructions.
  pub index: u8,
  /// Instructions
  pub instructions: Vector<InstructionJsonGeneric>,
}

/// With decoded JSON data.
#[derive(Debug, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct InstructionJsonParsedOverall {
  /// Known program name
  pub program: Option<SolanaProgramName>,
  /// Program Base58 identifier.
  pub program_id: SolanaAddressHashStr,
  /// Parsed instruction.
  pub parsed: Option<InstructionJsonParsedDecoded>,
}

/// Basic decoded instruction that may have a known information.
#[derive(Debug, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct InstructionJsonParsedDecoded {
  /// Information
  pub info: InstructionJsonParsedInfo,
  /// Type
  pub r#type: ArrayString<32>,
}

/// With decoded JSON data.
#[derive(Debug, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct InstructionPartiallyDecoded {
  /// Program Base58 identifier.
  pub program_id: SolanaAddressHashStr,
  /// Instruction accounts
  pub accounts: Vector<SolanaAddressHashStr>,
  /// Raw data
  pub data: String,
}

/// Decoded block message.
#[derive(Debug, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MessageJson<AK, I> {
  /// All block accounts.
  pub account_keys: Vector<AK>,
  /// All block instructions.
  pub instructions: Vector<I>,
  /// Recent blockhash.
  pub recent_blockhash: SolanaBlockhashStr,
}

/// Account information.
#[derive(Debug, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MessageJsonAccountKey {
  /// Base58 identifier.
  pub pubkey: SolanaAddressHashStr,
  /// Signed the transaction.
  pub signer: bool,
  /// Had state modified.
  pub writable: bool,
}

/// Decoded transaction
#[derive(Debug, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TransactionJson<AK, I> {
  /// All block signatures.
  pub signatures: Vector<SolanaSignatureHashStr>,
  /// Message
  pub message: MessageJson<AK, I>,
}
