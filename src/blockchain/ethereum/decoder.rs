use crate::blockchain::ethereum::{Word, misc::word_idx};
use core::slice::SliceIndex;

/// ABI decoder
#[derive(Debug)]
pub struct Decoder<'de> {
  bytes: &'de [u8],
  tail_idx: usize,
}

impl<'de> Decoder<'de> {
  pub(crate) fn new(bytes: &'de [u8]) -> Self {
    Self { bytes, tail_idx: 0 }
  }

  pub(crate) fn bytes<I>(&self, index: I) -> crate::Result<&'de I::Output>
  where
    I: SliceIndex<[u8]>,
  {
    self.bytes.get(index).ok_or(crate::Error::UnknownDecodingBytes)
  }

  pub(crate) fn tail_bytes(&self, len: usize) -> crate::Result<&'de [u8]> {
    let end = self.tail_idx.checked_add(len).ok_or(crate::Error::UnknownDecodingBytes)?;
    self.bytes(self.tail_idx..end)
  }

  pub(crate) fn take_tail<'local_de>(&mut self) -> crate::Result<Decoder<'local_de>>
  where
    'de: 'local_de,
  {
    Ok(Decoder::new(
      self
        .bytes
        .get(self.take_tail_idx()?..)
        .ok_or(crate::Error::WordIdxDoesNotHaveCorrespondingBytes)?,
    ))
  }

  pub(crate) fn take_tail_idx(&mut self) -> crate::Result<usize> {
    word_idx(&self.take_word()?)
  }

  pub(crate) fn take_word(&mut self) -> crate::Result<Word> {
    #[rustfmt::skip]
    let [
      b0, b1, b2, b3, b4, b5, b6, b7, b8, b9, b10, b11, b12, b13, b14, b15, b16, b17, b18, b19,
      b20, b21, b22, b23, b24, b25, b26, b27, b28, b29, b30, b31,
      ref rest @ ..,
    ] = *self.bytes
    else {
      panic!();
    };
    self.bytes = rest;
    Ok(Word([
      b0, b1, b2, b3, b4, b5, b6, b7, b8, b9, b10, b11, b12, b13, b14, b15, b16, b17, b18, b19,
      b20, b21, b22, b23, b24, b25, b26, b27, b28, b29, b30, b31,
    ]))
  }
}
