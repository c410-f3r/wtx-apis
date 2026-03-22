use crate::blockchain::solana::{
  InstructionAccountInput,
  misc::next_iter_elem,
  program::{AccountAddress, PackAccounts, TestingInstances},
};
use wtx::collection::Vector;

create_account_struct! {
  #[derive(Debug, PartialEq)]
  pub struct CloseAccountAccounts<A> {
    pub account_pubkey: A,
    pub destination_pubkey: A,
    pub owner_pubkey: A,
  }
}

impl<A> PackAccounts<A> for CloseAccountAccounts<A>
where
  A: AccountAddress,
{
  fn len(&self) -> usize {
    3
  }

  fn push_accounts(&self, vec: &mut Vector<InstructionAccountInput>) -> crate::Result<()>
  where
    A: AccountAddress,
  {
    let Self { account_pubkey, destination_pubkey, owner_pubkey } = self;
    vec.push(InstructionAccountInput::writable(account_pubkey.to_account_address()?))?;
    vec.push(InstructionAccountInput::writable(destination_pubkey.to_account_address()?))?;
    vec.push(InstructionAccountInput::sign(owner_pubkey.to_account_address()?))?;
    Ok(())
  }

  fn unpack_accounts(iter: &mut impl Iterator<Item = A>) -> crate::Result<Self> {
    Ok(Self {
      account_pubkey: next_iter_elem(iter)?,
      destination_pubkey: next_iter_elem(iter)?,
      owner_pubkey: next_iter_elem(iter)?,
    })
  }
}

impl TestingInstances for CloseAccountAccounts<[u8; 32]> {
  fn min_instance() -> crate::Result<Self> {
    Ok(Self { account_pubkey: [2; 32], destination_pubkey: [3; 32], owner_pubkey: [4; 32] })
  }
}
