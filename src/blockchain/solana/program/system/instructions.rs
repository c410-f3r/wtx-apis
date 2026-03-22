use crate::{
  blockchain::solana::program::system::{
    accounts::{CreateAccountAccounts, CreateAccountWithSeedAccounts, TransferAccounts},
    data::{CreateAccountParams, CreateAccountWithSeedParams, Instruction, TransferParams},
  },
  misc::HashArray32Unit,
};

create_and_impl_build_ix_input!(
  CreateAccount,
  CreateAccountAccounts<A>,
  CreateAccountParams<HashArray32Unit>,
  Instruction,
  Instruction::CreateAccount
);

create_and_impl_build_ix_input!(
  CreateAccountWithSeed,
  CreateAccountWithSeedAccounts<A>,
  CreateAccountWithSeedParams<HashArray32Unit>,
  Instruction,
  Instruction::CreateAccountWithSeed
);

create_and_impl_build_ix_input!(
  Transfer,
  TransferAccounts<A>,
  TransferParams,
  Instruction,
  Instruction::Transfer
);
