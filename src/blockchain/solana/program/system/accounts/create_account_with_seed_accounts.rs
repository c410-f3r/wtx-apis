use crate::blockchain::solana::{
  InstructionAccountInput,
  misc::next_iter_elem,
  program::{AccountAddress, PackAccounts, TestingInstances},
};
use wtx::collection::Vector;

create_account_struct! {
  #[derive(Debug, PartialEq)]
  pub struct CreateAccountWithSeedAccounts<A> {
    pub from_pubkey: A,
    pub to_pubkey: A,
    pub base: A
  }
}

impl<A> PackAccounts<A> for CreateAccountWithSeedAccounts<A>
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
    let Self { from_pubkey, to_pubkey, base } = self;
    vec.push(InstructionAccountInput::sign_and_writable(from_pubkey.to_account_address()?))?;
    vec.push(InstructionAccountInput::writable(to_pubkey.to_account_address()?))?;
    vec.push(InstructionAccountInput::sign(base.to_account_address()?))?;
    Ok(())
  }

  fn unpack_accounts(iter: &mut impl Iterator<Item = A>) -> crate::Result<Self> {
    Ok(Self {
      from_pubkey: next_iter_elem(iter)?,
      to_pubkey: next_iter_elem(iter)?,
      base: next_iter_elem(iter)?,
    })
  }
}

impl TestingInstances for CreateAccountWithSeedAccounts<[u8; 32]> {
  fn min_instance() -> crate::Result<Self> {
    Ok(Self { from_pubkey: [1; 32], to_pubkey: [2; 32], base: [3; 32] })
  }
}
