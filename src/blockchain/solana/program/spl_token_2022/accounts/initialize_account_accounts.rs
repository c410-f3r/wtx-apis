use crate::blockchain::solana::{
  InstructionAccountInput,
  misc::next_iter_elem,
  program::{AccountAddress, PackAccounts, TestingInstances},
};
use wtx::collection::Vector;

create_account_struct! {
  #[derive(Debug, PartialEq)]
  pub struct InitializeAccountAccounts<A> {
    pub account: A,
    pub mint: A,
    pub owner: A,
    pub rent_sysvar: A,
  }
}

impl<A> PackAccounts<A> for InitializeAccountAccounts<A>
where
  A: AccountAddress,
{
  fn len(&self) -> usize {
    4
  }

  fn push_accounts(&self, vec: &mut Vector<InstructionAccountInput>) -> crate::Result<()>
  where
    A: AccountAddress,
  {
    let Self { account, mint, owner, rent_sysvar } = self;
    vec.extend_from_copyable_slice(&[
      InstructionAccountInput::writable(account.to_account_address()?),
      InstructionAccountInput::none(mint.to_account_address()?),
      InstructionAccountInput::none(owner.to_account_address()?),
      InstructionAccountInput::none(rent_sysvar.to_account_address()?),
    ])?;
    Ok(())
  }

  fn unpack_accounts(iter: &mut impl Iterator<Item = A>) -> crate::Result<Self> {
    Ok(Self {
      account: next_iter_elem(iter)?,
      mint: next_iter_elem(iter)?,
      owner: next_iter_elem(iter)?,
      rent_sysvar: next_iter_elem(iter)?,
    })
  }
}

impl TestingInstances for InitializeAccountAccounts<[u8; 32]> {
  fn min_instance() -> crate::Result<Self> {
    Ok(Self { account: [1; 32], mint: [1; 32], owner: [1; 32], rent_sysvar: [1; 32] })
  }
}
