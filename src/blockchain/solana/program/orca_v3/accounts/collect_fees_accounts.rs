use crate::blockchain::solana::{
  InstructionAccountInput,
  misc::next_iter_elem,
  program::{AccountAddress, PackAccounts, TestingInstances},
};
use wtx::collection::Vector;

create_account_struct! {
  #[derive(Debug, Eq, PartialEq)]
  pub struct CollectFeesAccounts<A> {
    pub whirlpool: A,
    pub position_authority: A,
    pub position: A,
    pub position_token_account: A,
    pub token_owner_account_a: A,
    pub token_vault_a: A,
    pub token_owner_account_b: A,
    pub token_vault_b: A,
    pub token_program: A,
  }
}

impl<A> PackAccounts<A> for CollectFeesAccounts<A>
where
  A: AccountAddress,
{
  fn len(&self) -> usize {
    9
  }

  fn push_accounts(&self, vec: &mut Vector<InstructionAccountInput>) -> crate::Result<()>
  where
    A: AccountAddress,
  {
    let Self {
      whirlpool,
      position_authority,
      position,
      position_token_account,
      token_owner_account_a,
      token_vault_a,
      token_owner_account_b,
      token_vault_b,
      token_program,
    } = self;
    vec.push(InstructionAccountInput::writable(whirlpool.to_account_address()?))?;
    vec.push(InstructionAccountInput::sign(position_authority.to_account_address()?))?;
    vec.push(InstructionAccountInput::writable(position.to_account_address()?))?;
    vec.push(InstructionAccountInput::none(position_token_account.to_account_address()?))?;
    vec.push(InstructionAccountInput::writable(token_owner_account_a.to_account_address()?))?;
    vec.push(InstructionAccountInput::writable(token_vault_a.to_account_address()?))?;
    vec.push(InstructionAccountInput::writable(token_owner_account_b.to_account_address()?))?;
    vec.push(InstructionAccountInput::writable(token_vault_b.to_account_address()?))?;
    vec.push(InstructionAccountInput::none(token_program.to_account_address()?))?;
    Ok(())
  }

  fn unpack_accounts(iter: &mut impl Iterator<Item = A>) -> crate::Result<Self> {
    Ok(Self {
      whirlpool: next_iter_elem(iter)?,
      position_authority: next_iter_elem(iter)?,
      position: next_iter_elem(iter)?,
      position_token_account: next_iter_elem(iter)?,
      token_owner_account_a: next_iter_elem(iter)?,
      token_vault_a: next_iter_elem(iter)?,
      token_owner_account_b: next_iter_elem(iter)?,
      token_vault_b: next_iter_elem(iter)?,
      token_program: next_iter_elem(iter)?,
    })
  }
}

impl TestingInstances for CollectFeesAccounts<[u8; 32]> {
  fn min_instance() -> crate::Result<Self> {
    Ok(Self {
      whirlpool: [1; 32],
      position_authority: [2; 32],
      position: [3; 32],
      position_token_account: [4; 32],
      token_owner_account_a: [5; 32],
      token_vault_a: [6; 32],
      token_owner_account_b: [7; 32],
      token_vault_b: [8; 32],
      token_program: [9; 32],
    })
  }
}
