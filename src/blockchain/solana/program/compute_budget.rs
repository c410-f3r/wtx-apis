//! Manages compute limits and priority fees.

use crate::misc::HashArray32Static;

/// Data stored in the blockchain
pub mod data;
/// Program instructions
pub mod instructions;

/// Public address of the program
pub const ID: HashArray32Static = HashArray32Static::new(
  [
    3, 6, 70, 111, 229, 33, 23, 50, 255, 236, 173, 186, 114, 195, 155, 231, 188, 140, 229, 187,
    197, 247, 18, 107, 44, 67, 155, 58, 64, 0, 0, 0,
  ],
  "ComputeBudget111111111111111111111111111111",
);
