use crate::blockchain::solana::{
  InstructionAccountInput,
  misc::next_iter_elem,
  program::{AccountAddress, PackAccounts, TestingInstances},
};
use wtx::collection::Vector;

create_account_struct! {
  #[derive(Debug, PartialEq)]
  pub struct OpenPositionWithTokenExtensionsAccounts<A> {
    pub funder: A,
    pub owner: A,
    pub position: A,
    pub position_mint: A,
    pub position_token_account: A,
    pub whirlpool: A,
    pub token_2022_program: A,
    pub system_program: A,
    pub associated_token_program: A,
    pub metadata_update_auth: A,
  }
}

impl<A> PackAccounts<A> for OpenPositionWithTokenExtensionsAccounts<A>
where
  A: AccountAddress,
{
  fn len(&self) -> usize {
    10
  }

  fn push_accounts(&self, vec: &mut Vector<InstructionAccountInput>) -> crate::Result<()>
  where
    A: AccountAddress,
  {
    let Self {
      funder,
      owner,
      position,
      position_mint,
      position_token_account,
      whirlpool,
      token_2022_program,
      system_program,
      associated_token_program,
      metadata_update_auth,
    } = self;
    vec.push(InstructionAccountInput::sign_and_writable(funder.to_account_address()?))?;
    vec.push(InstructionAccountInput::none(owner.to_account_address()?))?;
    vec.push(InstructionAccountInput::writable(position.to_account_address()?))?;
    vec.push(InstructionAccountInput::sign_and_writable(position_mint.to_account_address()?))?;
    vec.push(InstructionAccountInput::writable(position_token_account.to_account_address()?))?;
    vec.push(InstructionAccountInput::none(whirlpool.to_account_address()?))?;
    vec.push(InstructionAccountInput::none(token_2022_program.to_account_address()?))?;
    vec.push(InstructionAccountInput::none(system_program.to_account_address()?))?;
    vec.push(InstructionAccountInput::none(associated_token_program.to_account_address()?))?;
    vec.push(InstructionAccountInput::none(metadata_update_auth.to_account_address()?))?;
    Ok(())
  }

  fn unpack_accounts(iter: &mut impl Iterator<Item = A>) -> crate::Result<Self> {
    Ok(Self {
      funder: next_iter_elem(iter)?,
      owner: next_iter_elem(iter)?,
      position: next_iter_elem(iter)?,
      position_mint: next_iter_elem(iter)?,
      position_token_account: next_iter_elem(iter)?,
      whirlpool: next_iter_elem(iter)?,
      token_2022_program: next_iter_elem(iter)?,
      system_program: next_iter_elem(iter)?,
      associated_token_program: next_iter_elem(iter)?,
      metadata_update_auth: next_iter_elem(iter)?,
    })
  }
}

impl TestingInstances for OpenPositionWithTokenExtensionsAccounts<[u8; 32]> {
  fn min_instance() -> crate::Result<Self> {
    Ok(Self {
      funder: [1; 32],
      owner: [2; 32],
      position: [3; 32],
      position_mint: [4; 32],
      position_token_account: [5; 32],
      whirlpool: [6; 32],
      token_2022_program: [7; 32],
      system_program: [8; 32],
      associated_token_program: [9; 32],
      metadata_update_auth: [10; 32],
    })
  }
}
