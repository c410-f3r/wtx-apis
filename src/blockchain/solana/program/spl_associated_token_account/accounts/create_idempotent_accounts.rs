use crate::blockchain::solana::{
  InstructionAccountInput,
  misc::next_iter_elem,
  program::{AccountAddress, PackAccounts, TestingInstances},
};
use wtx::collection::Vector;

create_account_struct! {
  #[derive(Debug, Eq, PartialEq)]
  pub struct CreateIdempotentAccounts<A> {
    pub funding_address: A,
    pub associated_account_address: A,
    pub wallet_address: A,
    pub token_mint_address: A,
    pub system_id: A,
    pub token_program_id: A,
  }
}

impl<A> PackAccounts<A> for CreateIdempotentAccounts<A>
where
  A: AccountAddress,
{
  fn len(&self) -> usize {
    6
  }

  fn push_accounts(&self, vec: &mut Vector<InstructionAccountInput>) -> crate::Result<()>
  where
    A: AccountAddress,
  {
    let Self {
      funding_address,
      associated_account_address,
      wallet_address,
      system_id,
      token_mint_address,
      token_program_id,
    } = self;
    vec.push(InstructionAccountInput::sign_and_writable(funding_address.to_account_address()?))?;
    vec
      .push(InstructionAccountInput::writable(associated_account_address.to_account_address()?))?;
    vec.push(InstructionAccountInput::none(wallet_address.to_account_address()?))?;
    vec.push(InstructionAccountInput::none(token_mint_address.to_account_address()?))?;
    vec.push(InstructionAccountInput::none(system_id.to_account_address()?))?;
    vec.push(InstructionAccountInput::none(token_program_id.to_account_address()?))?;
    Ok(())
  }

  fn unpack_accounts(iter: &mut impl Iterator<Item = A>) -> crate::Result<Self> {
    Ok(CreateIdempotentAccounts {
      funding_address: next_iter_elem(iter)?,
      associated_account_address: next_iter_elem(iter)?,
      wallet_address: next_iter_elem(iter)?,
      token_mint_address: next_iter_elem(iter)?,
      system_id: next_iter_elem(iter)?,
      token_program_id: next_iter_elem(iter)?,
    })
  }
}

impl TestingInstances for CreateIdempotentAccounts<[u8; 32]> {
  fn min_instance() -> crate::Result<Self> {
    Ok(Self {
      funding_address: [1; 32],
      associated_account_address: [1; 32],
      wallet_address: [2; 32],
      token_mint_address: [3; 32],
      system_id: [4; 32],
      token_program_id: [5; 32],
    })
  }
}
