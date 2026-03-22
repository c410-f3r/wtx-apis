//! Collections of programs used exclusively by the official JSON-RPC specification.

mod account_address;
pub mod address_lookup_table;
mod build_ix_input;
pub mod compute_budget;
mod generic_instruction_input;
mod len_bounds;
pub mod orca_v3;
mod pack_accounts;
mod pack_data;
mod program_error;
pub mod rent_sysvar;
mod slot_hashes;
pub mod spl_associated_token_account;
pub mod spl_memo;
pub mod spl_token;
pub mod spl_token_2022;
mod std_data;
pub mod system;
pub mod sysvar;
mod testing_instances;
#[cfg(test)]
mod tests;

pub use account_address::AccountAddress;
pub use build_ix_input::*;
pub use generic_instruction_input::GenericInstructionInput;
pub use len_bounds::*;
pub use pack_accounts::*;
pub use pack_data::*;
pub use program_error::ProgramError;
pub use slot_hashes::{SlotHash, SlotHashes};
pub use std_data::*;
pub use testing_instances::TestingInstances;

/// Maximum number of accounts that can sign a transaction
pub const MAX_SIGNERS: usize = 4;
