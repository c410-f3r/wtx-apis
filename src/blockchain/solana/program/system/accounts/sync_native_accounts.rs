use crate::blockchain::solana::{
  InstructionAccountInput,
  misc::next_iter_elem,
  program::{AccountAddress, PackAccounts, TestingInstances},
};
use wtx::collection::Vector;

create_account_struct! {
  #[derive(Debug, PartialEq)]
  pub struct SyncNativeAccounts<A> {
    pub account: A,
  }
}

impl<A> PackAccounts<A> for SyncNativeAccounts<A>
where
  A: AccountAddress,
{
  fn len(&self) -> usize {
    1
  }

  fn push_accounts(&self, vec: &mut Vector<InstructionAccountInput>) -> crate::Result<()>
  where
    A: AccountAddress,
  {
    let Self { account } = self;
    vec.push(InstructionAccountInput::writable(account.to_account_address()?))?;
    Ok(())
  }

  fn unpack_accounts(iter: &mut impl Iterator<Item = A>) -> crate::Result<Self> {
    Ok(Self { account: next_iter_elem(iter)? })
  }
}

impl TestingInstances for SyncNativeAccounts<[u8; 32]> {
  fn min_instance() -> crate::Result<Self> {
    Ok(Self { account: [1; 32] })
  }
}
