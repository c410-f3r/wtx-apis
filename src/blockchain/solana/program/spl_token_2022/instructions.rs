use crate::blockchain::solana::program::{
  spl_token_2022::{
    accounts::{
      CloseAccountAccounts, GetAccountDataSizeAccounts, InitializeAccountAccounts,
      TransferAccounts, TransferCheckedAccounts,
    },
    data::{Instruction, TransferCheckedParams, TransferParams},
  },
  system::accounts::SyncNativeAccounts,
};

create_and_impl_build_ix_input!(CloseAccount, CloseAccountAccounts<A>, (), Instruction, |_| {
  Instruction::CloseAccount
});

create_and_impl_build_ix_input!(
  InitializeAccount,
  InitializeAccountAccounts<A>,
  (),
  Instruction,
  |_| Instruction::InitializeAccount
);

create_and_impl_build_ix_input!(
  GetAccountDataSize,
  GetAccountDataSizeAccounts<A>,
  (),
  Instruction,
  |_| Instruction::GetAccountDataSize
);

create_and_impl_build_ix_input!(SyncNative, SyncNativeAccounts<A>, (), Instruction, |_| {
  Instruction::SyncNative
});

create_and_impl_build_ix_input!(
  Transfer,
  TransferAccounts<A>,
  TransferParams,
  Instruction,
  Instruction::Transfer
);

create_and_impl_build_ix_input!(
  TransferChecked,
  TransferCheckedAccounts<A>,
  TransferCheckedParams,
  Instruction,
  Instruction::TransferChecked
);
