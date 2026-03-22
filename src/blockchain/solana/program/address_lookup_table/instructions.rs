use crate::{
  blockchain::solana::program::address_lookup_table::{
    accounts::{
      CloseLookupTableAccounts, CreateLookupTableAccounts, DeactivateLookupTableAccounts,
      ExtendLookupTableAccounts,
    },
    data::{Instruction, NewAddresses},
  },
  misc::HashArray32Unit,
};

create_and_impl_build_ix_input!(
  CloseLookupTable,
  CloseLookupTableAccounts<A>,
  (),
  Instruction,
  |_| Instruction::CloseLookupTable
);

create_and_impl_build_ix_input!(
  CreateLookupTable,
  CreateLookupTableAccounts<A>,
  (u64, u8),
  Instruction,
  Instruction::CreateLookupTable
);

create_and_impl_build_ix_input!(
  DeactivateLookupTable,
  DeactivateLookupTableAccounts<A>,
  (),
  Instruction,
  |_| Instruction::DeactivateLookupTable
);

create_and_impl_build_ix_input!(
  ExtendLookupTable,
  ExtendLookupTableAccounts<A>,
  NewAddresses<HashArray32Unit>,
  Instruction,
  Instruction::ExtendLookupTable
);
