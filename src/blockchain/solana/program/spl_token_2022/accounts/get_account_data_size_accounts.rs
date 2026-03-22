use crate::blockchain::solana::{
  InstructionAccountInput,
  misc::next_iter_elem,
  program::{AccountAddress, PackAccounts, TestingInstances},
};
use wtx::collection::Vector;

create_account_struct! {
  #[derive(Debug, PartialEq)]
  pub struct GetAccountDataSizeAccounts<A> {
    pub mint: A,
  }
}

impl<A> PackAccounts<A> for GetAccountDataSizeAccounts<A>
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
    let Self { mint } = self;
    vec.push(InstructionAccountInput::none(mint.to_account_address()?))?;
    Ok(())
  }

  fn unpack_accounts(iter: &mut impl Iterator<Item = A>) -> crate::Result<Self> {
    Ok(Self { mint: next_iter_elem(iter)? })
  }
}

impl TestingInstances for GetAccountDataSizeAccounts<[u8; 32]> {
  fn min_instance() -> crate::Result<Self> {
    Ok(Self { mint: [1; 32] })
  }
}
