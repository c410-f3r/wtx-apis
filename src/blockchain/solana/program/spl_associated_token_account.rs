//! Single account attached to an address and its assoaciated elements.

/// Accounts required to execute instructions
pub mod accounts;
/// Data stored in the blockchain
pub mod data;
/// Program instructions
pub mod instructions;

use crate::{
  blockchain::solana::{
    SolanaAddressHash,
    misc::{find_program_address, is_on_curve},
  },
  misc::HashArray32Static,
};

/// Public address of the program
pub const ID: HashArray32Static = HashArray32Static::new(
  [
    140, 151, 37, 143, 78, 36, 137, 241, 187, 61, 16, 41, 20, 142, 13, 131, 11, 90, 19, 153, 218,
    255, 16, 132, 4, 142, 123, 216, 219, 233, 248, 89,
  ],
  "ATokenGPvbdGVxr1b2hvZbsiqW5xWH25efTNsLJA8knL",
);

/// Derives the associated token account address for the given wallet address, token mint and token program id
pub fn get_associated_token_address(
  allow_owner_off_curve: bool,
  token_mint_address: &SolanaAddressHash,
  token_program_id: &SolanaAddressHash,
  wallet_address: &SolanaAddressHash,
) -> crate::Result<SolanaAddressHash> {
  if !allow_owner_off_curve && !is_on_curve(wallet_address)? {
    return Err(crate::Error::SolanaTokenOwnerOffCurveError);
  }
  Ok(
    get_associated_token_address_and_bump_seed(
      ID.bytes(),
      token_mint_address,
      token_program_id,
      wallet_address,
    )?
    .0,
  )
}

fn get_associated_token_address_and_bump_seed(
  program_id: &SolanaAddressHash,
  token_mint_address: &SolanaAddressHash,
  token_program_id: &SolanaAddressHash,
  wallet_address: &SolanaAddressHash,
) -> crate::Result<(SolanaAddressHash, u8)> {
  find_program_address(&[wallet_address, token_program_id, token_mint_address], program_id)
}
