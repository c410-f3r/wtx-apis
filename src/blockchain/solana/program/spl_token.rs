//! Standard SPL Token program.

mod account_balance;
mod account_state;
mod generic_account;
mod mint_account;
mod token_account;
mod transfer_checked_instruction;
mod transfer_instruction;

use crate::misc::HashArray32Static;
pub use account_balance::*;
pub use account_state::*;
pub use generic_account::*;
pub use mint_account::*;
pub use token_account::*;
pub use transfer_checked_instruction::*;
pub use transfer_instruction::*;

/// Public address of the program
pub const ID: HashArray32Static = HashArray32Static::new(
  [
    6, 221, 246, 225, 215, 101, 161, 147, 217, 203, 225, 70, 206, 235, 121, 172, 28, 180, 133, 237,
    95, 91, 55, 145, 58, 140, 245, 133, 126, 255, 0, 169,
  ],
  "TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA",
);
