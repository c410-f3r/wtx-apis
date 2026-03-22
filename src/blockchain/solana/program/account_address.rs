use crate::misc::Hash;
use wtx::collection::ArrayStringU8;

/// Trait for types that can be represented as a 32-byte Solana account address.
pub trait AccountAddress {
  /// Converts the instance into a 32-byte account address.
  fn to_account_address(&self) -> crate::Result<[u8; 32]>;
}

impl AccountAddress for () {
  fn to_account_address(&self) -> crate::Result<[u8; 32]> {
    Ok([0; 32])
  }
}

impl<A> AccountAddress for &A
where
  A: AccountAddress,
{
  fn to_account_address(&self) -> crate::Result<[u8; 32]> {
    (*self).to_account_address()
  }
}

impl AccountAddress for [u8; 32] {
  fn to_account_address(&self) -> crate::Result<[u8; 32]> {
    Ok(*self)
  }
}

impl<const N: usize> AccountAddress for ArrayStringU8<N> {
  fn to_account_address(&self) -> crate::Result<[u8; 32]> {
    Ok(Hash::from_base58_str_ref(self.as_str())?.into_bytes())
  }
}
