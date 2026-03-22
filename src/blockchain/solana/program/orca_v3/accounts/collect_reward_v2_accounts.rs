use crate::blockchain::solana::{
  InstructionAccountInput,
  misc::next_iter_elem,
  program::{AccountAddress, PackAccounts, TestingInstances},
};
use wtx::collection::Vector;

create_account_struct! {
  #[derive(Debug, Eq, PartialEq)]
  pub struct CollectRewardV2Accounts<A> {
    pub whirlpool: A,
    pub position_authority: A,
    pub position: A,
    pub position_token_account: A,
    pub reward_owner_account: A,
    pub reward_mint: A,
    pub reward_vault: A,
    pub reward_token_program: A,
    pub memo_program: A,
  }
}

impl<A> PackAccounts<A> for CollectRewardV2Accounts<A>
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
      reward_owner_account,
      reward_mint,
      reward_vault,
      reward_token_program,
      memo_program,
    } = self;
    vec.push(InstructionAccountInput::none(whirlpool.to_account_address()?))?;
    vec.push(InstructionAccountInput::sign(position_authority.to_account_address()?))?;
    vec.push(InstructionAccountInput::writable(position.to_account_address()?))?;
    vec.push(InstructionAccountInput::none(position_token_account.to_account_address()?))?;
    vec.push(InstructionAccountInput::writable(reward_owner_account.to_account_address()?))?;
    vec.push(InstructionAccountInput::none(reward_mint.to_account_address()?))?;
    vec.push(InstructionAccountInput::writable(reward_vault.to_account_address()?))?;
    vec.push(InstructionAccountInput::none(reward_token_program.to_account_address()?))?;
    vec.push(InstructionAccountInput::none(memo_program.to_account_address()?))?;
    Ok(())
  }

  fn unpack_accounts(iter: &mut impl Iterator<Item = A>) -> crate::Result<Self> {
    Ok(Self {
      whirlpool: next_iter_elem(iter)?,
      position_authority: next_iter_elem(iter)?,
      position: next_iter_elem(iter)?,
      position_token_account: next_iter_elem(iter)?,
      reward_owner_account: next_iter_elem(iter)?,
      reward_mint: next_iter_elem(iter)?,
      reward_vault: next_iter_elem(iter)?,
      reward_token_program: next_iter_elem(iter)?,
      memo_program: next_iter_elem(iter)?,
    })
  }
}

impl TestingInstances for CollectRewardV2Accounts<[u8; 32]> {
  fn min_instance() -> crate::Result<Self> {
    Ok(Self {
      whirlpool: [1; 32],
      position_authority: [2; 32],
      position: [3; 32],
      position_token_account: [4; 32],
      reward_owner_account: [5; 32],
      reward_mint: [6; 32],
      reward_vault: [7; 32],
      reward_token_program: [8; 32],
      memo_program: [9; 32],
    })
  }
}
