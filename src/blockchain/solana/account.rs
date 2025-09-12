use crate::blockchain::solana::{
  Epoch, SolanaAddressHashStr, SolanaProgramName, program::spl_token::GenericAccount,
};
use alloc::{rc::Rc, string::String};
use core::cell::RefCell;

/// Generic account data representation.
#[
  // Data format is specified by the blockchain
  allow(clippy::large_enum_variant, variant_size_differences)
]
#[derive(Debug, serde::Deserialize)]
#[serde(rename_all = "camelCase", untagged)]
pub enum AccountData {
  /// Raw bytes
  Binary(String, AccountEncoding),
  /// Json
  Json(AccountDataJson),
}

/// Basic universal account information.
#[derive(Debug, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Account {
  /// Account data
  pub data: AccountData,
  /// If this account is a read-only program.
  pub executable: bool,
  /// Lamports assigned to this account.
  pub lamports: u64,
  /// Baser58 identifier of this account's owner.
  pub owner: SolanaAddressHashStr,
  /// The epoch at which this account will next owe rent.
  pub rent_epoch: Epoch,
}

/// Account json representation with additional metadata.
#[derive(Debug, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AccountDataJson {
  /// Data payload.
  pub parsed: AccountDataJsonParsed,
  /// Base58 identifier.
  pub program: SolanaProgramName,
  /// Data length
  pub space: u64,
}

/// Data payload of [AccountDataJson].
#[derive(Debug, serde::Deserialize)]
#[serde(rename_all = "camelCase", untagged)]
pub enum AccountDataJsonParsed {
  /// SPL token account
  SplTokenAccount(GenericAccount),
  /// Unknown program
  #[serde(deserialize_with = "crate::misc::deserialize_ignore_any")]
  Unknown,
}

/// Types of data representation of an account.
#[derive(Clone, Copy, Debug, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub enum AccountEncoding {
  /// Represents binary data in alphanumeric text.
  Base58,
  /// Represents binary data in sequences of 24 bits.
  Base64,
  /// Json representation with additional metadata.
  JsonParsed,
  /// Compressed base64 representation.
  #[serde(rename = "base64+zstd")]
  Base64Zstd,
}

/// Account in Solana programs
#[derive(Clone, Debug)]
#[repr(C)]
pub struct ProgramAccount<'any> {
  /// Public key of the account
  pub key: &'any [u8; 32],
  /// The lamports in the account.  Modifiable by programs.
  pub lamports: Rc<RefCell<&'any mut u64>>,
  /// The data held in this account.  Modifiable by programs.
  pub data: Rc<RefCell<&'any mut [u8]>>,
  /// Program that owns this account
  pub owner: &'any [u8; 32],
  /// The epoch at which this account will next owe rent
  pub rent_epoch: Epoch,
  /// Was the transaction signed by this account's public key?
  pub is_signer: bool,
  /// Is the account writable?
  pub is_writable: bool,
  /// This account's data contains a loaded program (and is now read-only)
  pub executable: bool,
}
