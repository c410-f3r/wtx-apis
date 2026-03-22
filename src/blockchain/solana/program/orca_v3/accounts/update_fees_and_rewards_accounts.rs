use crate::blockchain::solana::{
  InstructionAccountInput,
  misc::next_iter_elem,
  program::{AccountAddress, PackAccounts, TestingInstances},
};
use wtx::collection::Vector;

create_account_struct! {
  #[derive(Debug, PartialEq)]
  pub struct UpdateFeesAndRewardsAccounts<A> {
    pub whirlpool: A,
    pub position: A,
    pub tick_array_lower: A,
    pub tick_array_upper: A,
  }
}

impl<A> PackAccounts<A> for UpdateFeesAndRewardsAccounts<A>
where
  A: AccountAddress,
{
  fn len(&self) -> usize {
    4
  }

  fn push_accounts(&self, vec: &mut Vector<InstructionAccountInput>) -> crate::Result<()>
  where
    A: AccountAddress,
  {
    let Self { whirlpool, position, tick_array_lower, tick_array_upper } = self;
    vec.push(InstructionAccountInput::sign(whirlpool.to_account_address()?))?;
    vec.push(InstructionAccountInput::writable(position.to_account_address()?))?;
    vec.push(InstructionAccountInput::writable(tick_array_lower.to_account_address()?))?;
    vec.push(InstructionAccountInput::writable(tick_array_upper.to_account_address()?))?;
    Ok(())
  }

  fn unpack_accounts(iter: &mut impl Iterator<Item = A>) -> crate::Result<Self> {
    Ok(Self {
      whirlpool: next_iter_elem(iter)?,
      position: next_iter_elem(iter)?,
      tick_array_lower: next_iter_elem(iter)?,
      tick_array_upper: next_iter_elem(iter)?,
    })
  }
}

impl TestingInstances for UpdateFeesAndRewardsAccounts<[u8; 32]> {
  fn min_instance() -> crate::Result<Self> {
    Ok(Self {
      whirlpool: [1; 32],
      position: [2; 32],
      tick_array_lower: [3; 32],
      tick_array_upper: [4; 32],
    })
  }
}
