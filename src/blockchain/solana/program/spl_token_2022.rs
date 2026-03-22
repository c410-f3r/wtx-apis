//! SPL Token program with extensions.

/// Accounts required to execute instructions
pub mod accounts;
/// Data stored in the blockchain
pub mod data;
/// Program instructions
pub mod instructions;

use crate::misc::HashArray32Static;

/// Public address of the program
pub const ID: HashArray32Static = HashArray32Static::new(
  [
    6, 221, 246, 225, 238, 117, 143, 222, 24, 66, 93, 188, 228, 108, 205, 218, 182, 26, 252, 77,
    131, 185, 13, 39, 254, 189, 249, 40, 216, 161, 139, 252,
  ],
  "TokenzQdBNbLqP5VEhdkAS6EPFLC1PHnBqCXEpPxuEb",
);
