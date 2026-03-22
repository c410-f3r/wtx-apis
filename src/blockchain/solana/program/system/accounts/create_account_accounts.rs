use crate::blockchain::solana::{
  InstructionAccountInput,
  misc::next_iter_elem,
  program::{AccountAddress, PackAccounts, TestingInstances},
};
use wtx::collection::Vector;

create_account_struct! {
  #[derive(Debug, PartialEq)]
  pub struct CreateAccountAccounts<A> {
    pub from: A,
    pub to: A,
  }
}

impl<A> PackAccounts<A> for CreateAccountAccounts<A>
where
  A: AccountAddress,
{
  fn len(&self) -> usize {
    2
  }

  fn push_accounts(&self, vec: &mut Vector<InstructionAccountInput>) -> crate::Result<()>
  where
    A: AccountAddress,
  {
    let Self { from, to } = self;
    vec.push(InstructionAccountInput::sign_and_writable(from.to_account_address()?))?;
    vec.push(InstructionAccountInput::sign_and_writable(to.to_account_address()?))?;
    Ok(())
  }

  fn unpack_accounts(iter: &mut impl Iterator<Item = A>) -> crate::Result<Self> {
    Ok(Self { from: next_iter_elem(iter)?, to: next_iter_elem(iter)? })
  }
}

impl TestingInstances for CreateAccountAccounts<[u8; 32]> {
  fn min_instance() -> crate::Result<Self> {
    Ok(Self { from: [1; 32], to: [1; 32] })
  }
}
