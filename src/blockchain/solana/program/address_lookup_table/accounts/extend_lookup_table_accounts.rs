use crate::blockchain::solana::{
  InstructionAccountInput,
  misc::next_iter_elem,
  program::{AccountAddress, PackAccounts, TestingInstances},
};
use wtx::collection::Vector;

create_account_struct! {
  #[derive(Debug, PartialEq)]
  pub struct ExtendLookupTableAccounts<A> {
    pub lookup_table_address: A,
    pub authority_address: A,
    pub payer_address: Option<A>,
    pub system_program: Option< A>,
  }
}

impl<A> PackAccounts<A> for ExtendLookupTableAccounts<A>
where
  A: AccountAddress,
{
  fn len(&self) -> usize {
    let mut sum: usize = 2;
    if self.payer_address.is_some() {
      sum = sum.wrapping_add(1);
    }
    if self.system_program.is_some() {
      sum = sum.wrapping_add(1);
    }
    sum
  }

  fn push_accounts(&self, vec: &mut Vector<InstructionAccountInput>) -> crate::Result<()>
  where
    A: AccountAddress,
  {
    let Self { lookup_table_address, authority_address, payer_address, system_program } = self;
    vec.push(InstructionAccountInput::writable(lookup_table_address.to_account_address()?))?;
    vec.push(InstructionAccountInput::sign(authority_address.to_account_address()?))?;
    if let (Some(a), Some(b)) = (payer_address, system_program) {
      vec.push(InstructionAccountInput::sign_and_writable(a.to_account_address()?))?;
      vec.push(InstructionAccountInput::none(b.to_account_address()?))?;
    }
    Ok(())
  }

  fn unpack_accounts(iter: &mut impl Iterator<Item = A>) -> crate::Result<Self> {
    Ok(Self {
      lookup_table_address: next_iter_elem(iter)?,
      authority_address: next_iter_elem(iter)?,
      payer_address: next_iter_elem(iter).ok(),
      system_program: next_iter_elem(iter).ok(),
    })
  }
}

impl TestingInstances for ExtendLookupTableAccounts<[u8; 32]> {
  fn min_instance() -> crate::Result<Self> {
    Ok(Self {
      lookup_table_address: [1; 32],
      authority_address: [2; 32],
      payer_address: None,
      system_program: None,
    })
  }
}
