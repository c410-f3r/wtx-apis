use crate::blockchain::solana::{InstructionAccountInput, program::AccountAddress};
use wtx::collection::Vector;

/// Trait for types that can pack accounts into instructions or unpack them from account lists.
pub trait PackAccounts<A>
where
  A: AccountAddress,
  Self: Sized,
{
  /// Returns the number of accounts represented by this type.
  fn len(&self) -> usize;

  /// Pushes the represented accounts into the provided instruction account vector.
  fn push_accounts(&self, vec: &mut Vector<InstructionAccountInput>) -> crate::Result<()>;

  /// Unpacks accounts from an iterator into an instance of the type.
  fn unpack_accounts(iter: &mut impl Iterator<Item = A>) -> crate::Result<Self>;
}

impl<A> PackAccounts<A> for ()
where
  A: AccountAddress,
{
  fn len(&self) -> usize {
    0
  }

  fn push_accounts(&self, _: &mut Vector<InstructionAccountInput>) -> crate::Result<()>
  where
    A: AccountAddress,
  {
    Ok(())
  }

  fn unpack_accounts(_: &mut impl Iterator<Item = A>) -> crate::Result<Self> {
    Ok(())
  }
}
