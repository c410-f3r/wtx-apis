use crate::blockchain::solana::program::orca_v3::{
  accounts::{
    ClosePositionWithTokenExtensionsAccounts, CollectFeesAccounts, CollectRewardV2Accounts,
    ModifyLiquidityAccounts, ModifyLiquidityV2Accounts, OpenPositionAccounts,
    OpenPositionWithTokenExtensionsAccounts, SwapV2Accounts, UpdateFeesAndRewardsAccounts,
  },
  data::{
    CollectRewardV2Params, DecreaseLiquidityParams, IncreaseLiquidityParams,
    IncreaseLiquidityV2Params, Instruction, OpenPositionParams,
    OpenPositionWithTokenExtensionsParams, SwapV2Params,
  },
};

create_and_impl_build_ix_input!(
  ClosePositionWithTokenExtensions,
  ClosePositionWithTokenExtensionsAccounts<A>,
  (),
  Instruction,
  |_| { Instruction::ClosePositionWithTokenExtensions }
);

create_and_impl_build_ix_input!(CollectFees, CollectFeesAccounts<A>, (), Instruction, |_| {
  Instruction::CollectFees
});

create_and_impl_build_ix_input!(
  CollectRewardV2,
  CollectRewardV2Accounts<A>,
  CollectRewardV2Params,
  Instruction,
  Instruction::CollectRewardV2
);

create_and_impl_build_ix_input!(
  DecreaseLiquidity,
  ModifyLiquidityAccounts<A>,
  DecreaseLiquidityParams,
  Instruction,
  Instruction::DecreaseLiquidity
);

create_and_impl_build_ix_input!(
  IncreaseLiquidity,
  ModifyLiquidityAccounts<A>,
  IncreaseLiquidityParams,
  Instruction,
  Instruction::IncreaseLiquidity
);

create_and_impl_build_ix_input!(
  IncreaseLiquidityV2,
  ModifyLiquidityV2Accounts<A>,
  IncreaseLiquidityV2Params,
  Instruction,
  Instruction::IncreaseLiquidityV2
);

create_and_impl_build_ix_input!(
  OpenPosition,
  OpenPositionAccounts<A>,
  OpenPositionParams,
  Instruction,
  Instruction::OpenPosition
);

create_and_impl_build_ix_input!(
  OpenPositionWithTokenExtensions,
  OpenPositionWithTokenExtensionsAccounts<A>,
  OpenPositionWithTokenExtensionsParams,
  Instruction,
  Instruction::OpenPositionWithTokenExtensions
);

create_and_impl_build_ix_input!(
  UpdateFeesAndRewards,
  UpdateFeesAndRewardsAccounts<A>,
  (),
  Instruction,
  |_| Instruction::UpdateFeesAndRewards
);

create_and_impl_build_ix_input!(
  Swap,
  SwapV2Accounts<A>,
  SwapV2Params,
  Instruction,
  Instruction::SwapV2
);
