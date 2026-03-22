//! Orca V3 (CLMM)

/// Accounts required to execute instructions
pub mod accounts;
/// Data stored in the blockchain
pub mod data;
/// Program instructions
pub mod instructions;
mod math;

use crate::{
  blockchain::solana::{SolanaAddressHash, misc::find_program_address},
  misc::HashArray32Static,
};
pub use math::*;
use wtx::collection::ArrayStringU8;

/// Public address of the program
pub const ID: HashArray32Static = HashArray32Static::new(
  [
    14, 3, 104, 95, 142, 144, 144, 83, 228, 88, 18, 28, 102, 245, 167, 106, 237, 199, 112, 106,
    161, 28, 130, 248, 170, 149, 42, 143, 43, 120, 121, 169,
  ],
  "whirLbMiicVdio4qvUfM5KAg6Ct8VwpYzGff3uctyCc",
);
/// Update authority for Whirlpool NFTs.
pub const NFT_UPDATE_AUTH: HashArray32Static = HashArray32Static::new(
  [
    38, 106, 102, 174, 221, 151, 229, 240, 68, 113, 214, 8, 228, 198, 153, 57, 215, 122, 157, 153,
    179, 205, 208, 147, 17, 137, 254, 107, 145, 10, 210, 105,
  ],
  "3axbTs2z5GBy6usVbNVoqEgZMng3vZvMnAoX29BFfwhr",
);
/// The number of ticks contained within a single tick array.
pub const TICK_ARRAY_SIZE: u16 = 88;

/// Derives the position account address for a given mint key.
pub fn position_account(
  mint_key: &SolanaAddressHash,
  program_id: &SolanaAddressHash,
) -> crate::Result<(SolanaAddressHash, u8)> {
  find_program_address(&[b"position", mint_key], program_id)
}

/// Returns an iterator of position account addresses derived from a set of token addresses.
pub fn positions<'iter_addr, 'pi_addr, I>(
  program_id: &'pi_addr SolanaAddressHash,
  token_addresses: I,
) -> impl Iterator<Item = crate::Result<(SolanaAddressHash, u8)>> + 'pi_addr + 'iter_addr
where
  'pi_addr: 'iter_addr,
  I: Iterator<Item = (u8, u8, &'iter_addr SolanaAddressHash)> + 'pi_addr + 'iter_addr,
{
  token_addresses.filter_map(|el| {
    let (amount, decimals, address) = el;
    if amount == 1 && decimals == 0 {
      return None;
    }
    Some(find_program_address(&[b"position", address], program_id))
  })
}

/// Derives the tick array public key for a given tick index.
pub fn tick_array_pubk_from_tick_idx(
  program_id: &SolanaAddressHash,
  tick_idx: i32,
  tick_spacing: u16,
  whirlpool: &SolanaAddressHash,
) -> crate::Result<SolanaAddressHash> {
  tick_array_pubk(program_id, tick_array_start_tick_idx(tick_idx, tick_spacing), whirlpool)
}

/// Derives a set of 5 tick array public keys centered around the given tick index.
pub fn tick_array_pubks_from_tick_idx(
  program_id: &SolanaAddressHash,
  tick_idx: i32,
  tick_spacing: u16,
  whirlpool: &SolanaAddressHash,
) -> crate::Result<[SolanaAddressHash; 5]> {
  let tick_array_start_tick_idx = tick_array_start_tick_idx(tick_idx, tick_spacing);
  let offset = i32::from(tick_spacing).wrapping_add(TICK_ARRAY_SIZE.into());
  let _2_offsets = offset.wrapping_mul(2);
  let tick_array_indcs = [
    tick_array_start_tick_idx,
    tick_array_start_tick_idx.wrapping_add(offset),
    tick_array_start_tick_idx.wrapping_add(_2_offsets),
    tick_array_start_tick_idx.wrapping_sub(offset),
    tick_array_start_tick_idx.wrapping_sub(_2_offsets),
  ];
  let tick_array_addresses = [
    tick_array_pubk(program_id, tick_array_indcs[0], whirlpool)?,
    tick_array_pubk(program_id, tick_array_indcs[1], whirlpool)?,
    tick_array_pubk(program_id, tick_array_indcs[2], whirlpool)?,
    tick_array_pubk(program_id, tick_array_indcs[3], whirlpool)?,
    tick_array_pubk(program_id, tick_array_indcs[4], whirlpool)?,
  ];
  Ok(tick_array_addresses)
}

fn tick_array_pubk(
  program_id: &SolanaAddressHash,
  start_tick_idx: i32,
  whirlpool: &SolanaAddressHash,
) -> crate::Result<SolanaAddressHash> {
  Ok(
    find_program_address(
      &[
        b"tick_array",
        whirlpool,
        ArrayStringU8::<10>::try_from(format_args!("{start_tick_idx}"))?.as_bytes(),
      ],
      program_id,
    )?
    .0,
  )
}

fn tick_array_start_tick_idx(tick_idx: i32, tick_spacing: u16) -> i32 {
  let tick_spacing_i32 = i32::from(tick_spacing);
  let real_index = tick_idx.div_euclid(tick_spacing_i32).div_euclid(TICK_ARRAY_SIZE.into());
  real_index.wrapping_mul(tick_spacing_i32).wrapping_mul(TICK_ARRAY_SIZE.into())
}

#[cfg(test)]
mod test {
  use crate::{
    blockchain::solana::program::orca_v3::{ID, tick_array_pubk, tick_array_pubk_from_tick_idx},
    misc::HashArray32ArrayString44,
  };

  #[test]
  fn tick_array_pubk_has_correct_output() {
    assert_eq!(
      tick_array_pubk(
        ID.bytes(),
        0,
        HashArray32ArrayString44::from_base58_str(
          "2kJmUjxWBwL2NGPBV2PiA5hWtmLCqcKY6reQgkrPtaeS".try_into().unwrap()
        )
        .unwrap()
        .bytes()
      )
      .unwrap(),
      *HashArray32ArrayString44::from_base58_str(
        "8PhPzk7n4wU98Z6XCbVtPai2LtXSxYnfjkmgWuoAU8Zy".try_into().unwrap()
      )
      .unwrap()
      .bytes(),
    );
  }

  #[test]
  fn tick_array_pubk_from_tick_idx_has_correct_output() {
    let pool = "HrvrhPtNq8JEGbi7dhMFuXy1Jms49nZrgC6GLjZ3cPyo".try_into().unwrap();
    let lower = tick_array_pubk_from_tick_idx(
      ID.bytes(),
      -30336,
      64,
      HashArray32ArrayString44::from_base58_str(pool).unwrap().bytes(),
    )
    .unwrap();
    let upper = tick_array_pubk_from_tick_idx(
      ID.bytes(),
      -29888,
      64,
      HashArray32ArrayString44::from_base58_str(pool).unwrap().bytes(),
    )
    .unwrap();
    assert_eq!(
      HashArray32ArrayString44::from_base58_bytes(lower).unwrap().str().as_str(),
      "6P8cAR8SLYkr1xyX9c98M6A8byUuSTvKYA8aYhc5pS3z"
    );
    assert_eq!(
      HashArray32ArrayString44::from_base58_bytes(upper).unwrap().str().as_str(),
      "6P8cAR8SLYkr1xyX9c98M6A8byUuSTvKYA8aYhc5pS3z"
    );
  }
}
