use crate::blockchain::solana::program::spl_associated_token_account::{
  accounts::CreateIdempotentAccounts, data::Instruction,
};

create_and_impl_build_ix_input!(
  CreateIdempotent,
  CreateIdempotentAccounts<A>,
  (),
  Instruction,
  |_| Instruction::CreateIdempotent
);
