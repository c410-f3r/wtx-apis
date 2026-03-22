use crate::blockchain::solana::{
  InstructionAccountInput,
  misc::next_iter_elem,
  program::{AccountAddress, PackAccounts, TestingInstances},
};
use wtx::collection::Vector;

create_account_struct! {
  #[derive(Debug, Eq, PartialEq)]
  pub struct ModifyLiquidityAccounts<A> {
    pub whirlpool: A,
    pub token_program: A,
    pub position_authority: A,
    pub position: A,
    pub position_token_account: A,
    pub token_owner_account_a: A,
    pub token_owner_account_b: A,
    pub token_vault_a: A,
    pub token_vault_b: A,
    pub tick_array_lower: A,
    pub tick_array_upper: A,
  }
}

impl<A> PackAccounts<A> for ModifyLiquidityAccounts<A>
where
  A: AccountAddress,
{
  fn len(&self) -> usize {
    11
  }

  fn push_accounts(&self, vec: &mut Vector<InstructionAccountInput>) -> crate::Result<()>
  where
    A: AccountAddress,
  {
    let Self {
      whirlpool,
      token_program,
      position_authority,
      position,
      position_token_account,
      token_owner_account_a,
      token_owner_account_b,
      token_vault_a,
      token_vault_b,
      tick_array_lower,
      tick_array_upper,
    } = self;
    vec.push(InstructionAccountInput::writable(whirlpool.to_account_address()?))?;
    vec.push(InstructionAccountInput::none(token_program.to_account_address()?))?;
    vec.push(InstructionAccountInput::sign(position_authority.to_account_address()?))?;
    vec.push(InstructionAccountInput::writable(position.to_account_address()?))?;
    vec.push(InstructionAccountInput::writable(position_token_account.to_account_address()?))?;
    vec.push(InstructionAccountInput::writable(token_owner_account_a.to_account_address()?))?;
    vec.push(InstructionAccountInput::writable(token_owner_account_b.to_account_address()?))?;
    vec.push(InstructionAccountInput::writable(token_vault_a.to_account_address()?))?;
    vec.push(InstructionAccountInput::writable(token_vault_b.to_account_address()?))?;
    vec.push(InstructionAccountInput::writable(tick_array_lower.to_account_address()?))?;
    vec.push(InstructionAccountInput::writable(tick_array_upper.to_account_address()?))?;
    Ok(())
  }

  fn unpack_accounts(iter: &mut impl Iterator<Item = A>) -> crate::Result<Self> {
    Ok(Self {
      whirlpool: next_iter_elem(iter)?,
      token_program: next_iter_elem(iter)?,
      position_authority: next_iter_elem(iter)?,
      position: next_iter_elem(iter)?,
      position_token_account: next_iter_elem(iter)?,
      token_owner_account_a: next_iter_elem(iter)?,
      token_owner_account_b: next_iter_elem(iter)?,
      token_vault_a: next_iter_elem(iter)?,
      token_vault_b: next_iter_elem(iter)?,
      tick_array_lower: next_iter_elem(iter)?,
      tick_array_upper: next_iter_elem(iter)?,
    })
  }
}

impl TestingInstances for ModifyLiquidityAccounts<[u8; 32]> {
  fn min_instance() -> crate::Result<Self> {
    Ok(Self {
      whirlpool: [1; 32],
      token_program: [2; 32],
      position_authority: [3; 32],
      position: [4; 32],
      position_token_account: [5; 32],
      token_owner_account_a: [6; 32],
      token_owner_account_b: [7; 32],
      token_vault_a: [8; 32],
      token_vault_b: [9; 32],
      tick_array_lower: [10; 32],
      tick_array_upper: [11; 32],
    })
  }
}
