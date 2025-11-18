use crate::blockchain::ethereum::{SolTy, misc::keccak256};
use wtx::collection::Vector;

/// Solidity Structure
pub trait SolStruct<'de>: SolTy<'de> {
  /// EIP-712 components.
  fn eip712_components() -> &'static [&'static str];

  /// Encodes itself using EIP-712 `encodeData`.
  fn eip712_encode_data(&self, buffer: &mut Vector<u8>) -> crate::Result<impl AsRef<[u8]>>;

  /// Return the root EIP-712 type
  fn eip712_encode_type() -> &'static str;

  /// Calculates the EIP-712 `hashStruct`.
  #[inline]
  fn eip712_hash_struct(&self, buffer: &mut Vector<u8>) -> crate::Result<[u8; 32]> {
    Ok(keccak256([&self.eip712_type_hash(), self.eip712_encode_data(buffer)?.as_ref()]))
  }

  /// Returns the root EIP-712 type
  fn eip712_root_type() -> &'static str;

  /// Calculates the EIP-712 `typeHash`.
  #[inline]
  fn eip712_type_hash(&self) -> [u8; 32] {
    keccak256([Self::eip712_encode_type().as_bytes()])
  }
}
