use crate::blockchain::ethereum::Word;
use sha3::{Digest, digest::FixedOutput};

pub(crate) fn has_only_zeros(data: &[u8]) -> bool {
  data.iter().all(|byte| *byte == 0)
}

pub(crate) fn keccak256<const N: usize>(slices: [&[u8]; N]) -> [u8; 32] {
  let mut hasher = sha3::Keccak256::default();
  for slice in slices {
    hasher.update(slice);
  }
  hasher.finalize_fixed().into()
}

pub(crate) fn pad_u64(value: u64) -> Word {
  let mut padded = [0; 32];
  padded[24..32].copy_from_slice(&value.to_be_bytes());
  Word(padded)
}

pub(crate) const fn padded_len(data: &[u8]) -> usize {
  next_multiple_of_32(data.len())
}

pub(crate) fn word_idx(word: &Word) -> crate::Result<usize> {
  let [.., a, b, c, d, e, f, g, h] = word.0;
  Ok(u64::from_be_bytes([a, b, c, d, e, f, g, h]).try_into().map_err(wtx::Error::from)?)
}

pub(crate) const fn words_for(data: &[u8]) -> usize {
  words_for_len(data.len())
}

pub(crate) const fn words_for_len(len: usize) -> usize {
  len.div_ceil(32)
}

const fn next_multiple_of_32(n: usize) -> usize {
  match n % 32 {
    0 => n,
    rest => n.wrapping_add(32usize.wrapping_sub(rest)),
  }
}
