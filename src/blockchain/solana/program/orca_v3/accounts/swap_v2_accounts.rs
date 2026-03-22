use crate::blockchain::solana::{
  InstructionAccountInput,
  misc::next_iter_elem,
  program::{AccountAddress, PackAccounts, TestingInstances},
};
use wtx::collection::Vector;

create_account_struct! {
  #[derive(Debug, PartialEq)]
  pub struct SwapV2Accounts<A> {
    pub token_program_a: A,
    pub token_program_b: A,
    pub memo_program: A,
    pub token_authority: A,
    pub whirlpool: A,
    pub token_mint_a: A,
    pub token_mint_b: A,
    pub token_owner_account_a: A,
    pub token_vault_a: A,
    pub token_owner_account_b: A,
    pub token_vault_b: A,
    pub tick_array_0: A,
    pub tick_array_1: A,
    pub tick_array_2: A,
    pub oracle: A,
    pub remaining_accounts: [A; 2]
  }
}

impl<A> PackAccounts<A> for SwapV2Accounts<A>
where
  A: AccountAddress,
{
  fn len(&self) -> usize {
    17
  }

  fn push_accounts(&self, vec: &mut Vector<InstructionAccountInput>) -> crate::Result<()>
  where
    A: AccountAddress,
  {
    let Self {
      token_program_a,
      token_program_b,
      memo_program,
      token_authority,
      whirlpool,
      token_mint_a,
      token_mint_b,
      token_owner_account_a,
      token_vault_a,
      token_owner_account_b,
      token_vault_b,
      tick_array_0,
      tick_array_1,
      tick_array_2,
      oracle,
      remaining_accounts: [a, b],
    } = self;
    vec.push(InstructionAccountInput::none(token_program_a.to_account_address()?))?;
    vec.push(InstructionAccountInput::none(token_program_b.to_account_address()?))?;
    vec.push(InstructionAccountInput::none(memo_program.to_account_address()?))?;
    vec.push(InstructionAccountInput::sign(token_authority.to_account_address()?))?;
    vec.push(InstructionAccountInput::writable(whirlpool.to_account_address()?))?;
    vec.push(InstructionAccountInput::none(token_mint_a.to_account_address()?))?;
    vec.push(InstructionAccountInput::none(token_mint_b.to_account_address()?))?;
    vec.push(InstructionAccountInput::writable(token_owner_account_a.to_account_address()?))?;
    vec.push(InstructionAccountInput::writable(token_vault_a.to_account_address()?))?;
    vec.push(InstructionAccountInput::writable(token_owner_account_b.to_account_address()?))?;
    vec.push(InstructionAccountInput::writable(token_vault_b.to_account_address()?))?;
    vec.push(InstructionAccountInput::writable(tick_array_0.to_account_address()?))?;
    vec.push(InstructionAccountInput::writable(tick_array_1.to_account_address()?))?;
    vec.push(InstructionAccountInput::writable(tick_array_2.to_account_address()?))?;
    vec.push(InstructionAccountInput::writable(oracle.to_account_address()?))?;
    vec.push(InstructionAccountInput::writable(a.to_account_address()?))?;
    vec.push(InstructionAccountInput::writable(b.to_account_address()?))?;
    Ok(())
  }

  fn unpack_accounts(iter: &mut impl Iterator<Item = A>) -> crate::Result<Self> {
    Ok(Self {
      token_program_a: next_iter_elem(iter)?,
      token_program_b: next_iter_elem(iter)?,
      memo_program: next_iter_elem(iter)?,
      token_authority: next_iter_elem(iter)?,
      whirlpool: next_iter_elem(iter)?,
      token_mint_a: next_iter_elem(iter)?,
      token_mint_b: next_iter_elem(iter)?,
      token_owner_account_a: next_iter_elem(iter)?,
      token_vault_a: next_iter_elem(iter)?,
      token_owner_account_b: next_iter_elem(iter)?,
      token_vault_b: next_iter_elem(iter)?,
      tick_array_0: next_iter_elem(iter)?,
      tick_array_1: next_iter_elem(iter)?,
      tick_array_2: next_iter_elem(iter)?,
      oracle: next_iter_elem(iter)?,
      remaining_accounts: [next_iter_elem(iter)?, next_iter_elem(iter)?],
    })
  }
}

impl TestingInstances for SwapV2Accounts<[u8; 32]> {
  fn min_instance() -> crate::Result<Self> {
    Ok(Self {
      token_program_a: [1; 32],
      token_program_b: [2; 32],
      memo_program: [3; 32],
      token_authority: [4; 32],
      whirlpool: [5; 32],
      token_mint_a: [6; 32],
      token_mint_b: [7; 32],
      token_owner_account_a: [8; 32],
      token_vault_a: [9; 32],
      token_owner_account_b: [10; 32],
      token_vault_b: [11; 32],
      tick_array_0: [12; 32],
      tick_array_1: [13; 32],
      tick_array_2: [14; 32],
      oracle: [15; 32],
      remaining_accounts: [[16; 32], [17; 32]],
    })
  }
}
