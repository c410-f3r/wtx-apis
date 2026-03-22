use crate::blockchain::solana::{
  InstructionAccountInput,
  misc::next_iter_elem,
  program::{AccountAddress, PackAccounts, TestingInstances},
};
use wtx::collection::Vector;

create_account_struct! {
  #[derive(Debug, PartialEq)]
  pub struct DeactivateLookupTableAccounts<A> {
    pub lookup_table_address: A,
    pub authority_address: A,
  }
}

impl<A> PackAccounts<A> for DeactivateLookupTableAccounts<A>
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
    let Self { lookup_table_address, authority_address } = self;
    vec.push(InstructionAccountInput::writable(lookup_table_address.to_account_address()?))?;
    vec.push(InstructionAccountInput::sign(authority_address.to_account_address()?))?;
    Ok(())
  }

  fn unpack_accounts(iter: &mut impl Iterator<Item = A>) -> crate::Result<Self> {
    Ok(Self {
      lookup_table_address: next_iter_elem(iter)?,
      authority_address: next_iter_elem(iter)?,
    })
  }
}

impl TestingInstances for DeactivateLookupTableAccounts<[u8; 32]> {
  fn min_instance() -> crate::Result<Self> {
    Ok(Self { lookup_table_address: [1; 32], authority_address: [2; 32] })
  }
}
