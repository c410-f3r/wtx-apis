use crate::blockchain::solana::{
  InstructionAccountInput,
  misc::next_iter_elem,
  program::{AccountAddress, PackAccounts, TestingInstances},
};
use wtx::collection::Vector;

create_account_struct! {
  #[derive(Debug, PartialEq)]
  pub struct ClosePositionWithTokenExtensionsAccounts<A> {
    pub position_authority: A,
    pub receiver: A,
    pub position: A,
    pub position_mint: A,
    pub position_token_account: A,
    pub token_2022_program: A,
  }
}

impl<A> PackAccounts<A> for ClosePositionWithTokenExtensionsAccounts<A>
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
      position_authority,
      receiver,
      position,
      position_mint,
      position_token_account,
      token_2022_program,
    } = self;
    vec.push(InstructionAccountInput::sign(position_authority.to_account_address()?))?;
    vec.push(InstructionAccountInput::writable(receiver.to_account_address()?))?;
    vec.push(InstructionAccountInput::writable(position.to_account_address()?))?;
    vec.push(InstructionAccountInput::writable(position_mint.to_account_address()?))?;
    vec.push(InstructionAccountInput::writable(position_token_account.to_account_address()?))?;
    vec.push(InstructionAccountInput::none(token_2022_program.to_account_address()?))?;
    Ok(())
  }

  fn unpack_accounts(iter: &mut impl Iterator<Item = A>) -> crate::Result<Self> {
    Ok(Self {
      position_authority: next_iter_elem(iter)?,
      receiver: next_iter_elem(iter)?,
      position: next_iter_elem(iter)?,
      position_mint: next_iter_elem(iter)?,
      position_token_account: next_iter_elem(iter)?,
      token_2022_program: next_iter_elem(iter)?,
    })
  }
}

impl TestingInstances for ClosePositionWithTokenExtensionsAccounts<[u8; 32]> {
  fn min_instance() -> crate::Result<Self> {
    Ok(Self {
      position_authority: [1; 32],
      receiver: [2; 32],
      position: [3; 32],
      position_mint: [4; 32],
      position_token_account: [5; 32],
      token_2022_program: [6; 32],
    })
  }
}
