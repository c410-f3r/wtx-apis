use crate::blockchain::ethereum::{Word, misc::pad_u64};
use wtx::{
  collection::{ArrayVectorU8, Vector},
  misc::Usize,
};

/// ABI encoder.
#[derive(Clone, Debug, Default)]
pub struct Encoder {
  buffer: Vector<u8>,
  last_tail_idx: usize,
  tail_idxs: ArrayVectorU8<usize, 4>,
}

impl Encoder {
  //pub(crate) fn from_buffer(mut buffer: Vector<u8>) -> Self {
  //  buffer.clear();
  //  Self { buffer, last_tail_idx: 0, tail_idxs: ArrayVectorU8::new() }
  //}

  #[cfg(test)]
  pub(crate) const fn new() -> Self {
    Self { buffer: Vector::new(), last_tail_idx: 0, tail_idxs: ArrayVectorU8::new() }
  }

  //pub(crate) const fn buffer_mut(&mut self) -> &mut Vector<u8> {
  //  &mut self.buffer
  //}

  pub(crate) fn bump_tail_idx(&mut self, words: usize) {
    if let Some(last) = self.tail_idxs.last_mut() {
      let tail_idx = last.wrapping_add(words.wrapping_mul(32));
      *last = tail_idx;
      self.last_tail_idx = tail_idx;
    }
  }

  pub(crate) fn pop_tail_idx(&mut self) -> Option<usize> {
    self.tail_idxs.pop()
  }

  pub(crate) fn push_last_tail_idx(&mut self) -> crate::Result<()> {
    self.push_words([pad_u64(Usize::from(self.last_tail_idx).into())])
  }

  pub(crate) fn push_packed_sequence(&mut self, bytes: &[u8]) -> crate::Result<()> {
    self.push_bytes(&pad_u64(Usize::from(bytes.len()).into()).0, bytes)
  }

  pub(crate) fn push_tail_idx_by_words(&mut self, words: usize) -> crate::Result<()> {
    let tail_idx = words.wrapping_mul(32);
    self.tail_idxs.push(tail_idx)?;
    self.last_tail_idx = tail_idx;
    Ok(())
  }

  pub(crate) fn push_words<I>(&mut self, words: I) -> crate::Result<()>
  where
    I: IntoIterator<Item = Word>,
    I::IntoIter: Clone,
  {
    let _ = self.buffer.extend_from_copyable_slices(words)?;
    Ok(())
  }

  fn push_bytes(&mut self, initial: &[u8], bytes: &[u8]) -> crate::Result<()> {
    let rem = bytes.len() % 32;
    let array = [0u8; 32];
    let _ = self.buffer.extend_from_copyable_slices([
      initial,
      bytes,
      if rem != 0 { array.get(rem..).unwrap_or_default() } else { &[] },
    ])?;
    Ok(())
  }
}

#[cfg(test)]
mod tests {
  use crate::blockchain::ethereum::{Word, encoder::Encoder};

  #[test]
  fn push_packed_sequence() {
    let mut encoder = Encoder::new();
    encoder.push_packed_sequence(&[3; 37]).unwrap();
    assert_eq!(
      &*encoder.buffer,
      &[
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 37, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3,
        3, 3, 3, 3, 3, 3, 3, 3, 3, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0
      ]
    );
  }

  #[test]
  fn push_packed_sequence_with_empty_bytes() {
    let mut encoder = Encoder::new();
    encoder.push_packed_sequence(&[]).unwrap();
    assert_eq!(&*encoder.buffer, &[0u8; 32]);
  }

  #[test]
  fn push_word() {
    let mut encoder = Encoder::new();
    encoder.push_words([Word([1; 32]), Word([2; 32])]).unwrap();
    assert_eq!(
      &*encoder.buffer,
      &[
        1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1,
        1, 1, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2,
        2, 2, 2, 2
      ]
    );
  }
}
