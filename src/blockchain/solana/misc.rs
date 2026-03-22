mod pack;
mod sha256_hasher;

use crate::blockchain::solana::SolanaAddressHash;
use core::slice::SliceIndex;
pub(crate) use pack::*;
use wtx::misc::Lease;

const MAX_SEED_LEN: usize = 32;
const PDA_MARKER: &[u8; 21] = b"ProgramDerivedAddress";
const MAX_SEEDS: usize = 16;

/// Derives a public key from a base key, a seed, and an owner program ID.
pub fn create_with_seed(
  from: &&[u8],
  seed: &str,
  owner: &[u8],
) -> crate::Result<SolanaAddressHash> {
  if seed.len() > MAX_SEED_LEN {
    return Err(crate::Error::SolanaMaxSeedLengthExceeded);
  }
  if let Some(start) = owner.len().checked_sub(PDA_MARKER.len())
    && owner.get(start..).unwrap_or_default() == PDA_MARKER
  {
    return Err(crate::Error::SolanaIllegalOwner);
  }
  let mut hasher = sha256_hasher::Sha256Hasher::default();
  hasher.hashv(&[from.lease(), seed.lease(), owner]);
  hasher.result()
}

/// Finds a valid program-derived address and its bump seed.
pub fn find_program_address(
  seeds: &[&[u8]],
  program_id: &SolanaAddressHash,
) -> crate::Result<(SolanaAddressHash, u8)> {
  for bump_seed in (1..=u8::MAX).rev() {
    match create_program_address(&[bump_seed], seeds, program_id) {
      Err(crate::Error::SolanaInvalidSeeds) => {}
      Ok(address) => return Ok((address, bump_seed)),
      _ => break,
    }
  }
  Err(crate::Error::SolanaNoSeedFound)
}

pub(crate) fn is_on_curve(hash: &[u8; 32]) -> crate::Result<bool> {
  #[cfg(not(target_os = "solana"))]
  return Ok(
    curve25519_dalek::edwards::CompressedEdwardsY::from_slice(hash)
      .map_err(wtx::Error::from)?
      .decompress()
      .is_some(),
  );
  #[cfg(target_os = "solana")]
  return Err(crate::Error::CanNotRunInSolanaEnvironment);
}

#[track_caller]
pub(crate) fn next_iter_elem<I>(iter: &mut I) -> crate::Result<I::Item>
where
  I: Iterator,
{
  iter.next().ok_or(crate::Error::IteratorIsExhausted)
}

pub(crate) fn sub_slice<I, T>(slice: &[T], idx: I) -> &I::Output
where
  I: SliceIndex<[T], Output = [T]>,
{
  slice.get(idx).unwrap_or_default()
}

fn create_program_address(
  bump_seed: &[u8],
  seeds: &[&[u8]],
  program_id: &[u8],
) -> crate::Result<SolanaAddressHash> {
  if seeds.len() > MAX_SEEDS {
    return Err(crate::Error::SolanaMaxSeedLengthExceeded);
  }

  for seed in seeds.iter().copied().chain([bump_seed]) {
    if seed.len() > MAX_SEED_LEN {
      return Err(crate::Error::SolanaMaxSeedLengthExceeded);
    }
  }

  let mut hasher = sha256_hasher::Sha256Hasher::default();
  for seed in seeds.iter().copied().chain([bump_seed]) {
    hasher.hash(seed);
  }
  hasher.hashv(&[program_id, PDA_MARKER]);

  let hash = hasher.result()?;
  if is_on_curve(&hash)? {
    return Err(crate::Error::SolanaInvalidSeeds);
  }

  Ok(hash)
}

/// `namespace` is usually named "global".
/// `name` is usually the struct's name in snake case.
fn _anchor_discriminator(namespace: &str, name: &str) -> crate::Result<[u8; 8]> {
  let preimage = alloc::format!("{namespace}:{name}");
  let mut sighash = [0u8; 8];
  let mut hasher = sha256_hasher::Sha256Hasher::default();
  hasher.hash(preimage.as_bytes());
  sighash.copy_from_slice(&hasher.result()?[..8]);
  Ok(sighash)
}
