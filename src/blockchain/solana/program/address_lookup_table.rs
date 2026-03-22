//! Allows the inclusion of more accounts in a transaction

use crate::{blockchain::solana::misc::find_program_address, misc::HashArray32Static};

/// Accounts required to execute instructions
pub mod accounts;
/// Data stored in the blockchain
pub mod data;
/// Program instructions
pub mod instructions;

/// Public address of the program
pub const ID: HashArray32Static = HashArray32Static::new(
  [
    2, 119, 166, 175, 151, 51, 155, 122, 200, 141, 24, 146, 201, 4, 70, 245, 0, 2, 48, 146, 102,
    246, 46, 83, 193, 24, 36, 73, 130, 0, 0, 0,
  ],
  "AddressLookupTab1e1111111111111111111111111",
);

/// Derives the address of a lookup table using an authority address and a recent slot.
pub fn derive_lookup_table_address(
  authority_address: &[u8; 32],
  recent_slot: u64,
) -> crate::Result<([u8; 32], u8)> {
  find_program_address(&[authority_address, &recent_slot.to_le_bytes()], ID.bytes())
}
