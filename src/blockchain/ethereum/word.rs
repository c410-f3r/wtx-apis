use wtx::misc::Lease;

use crate::blockchain::ethereum::{SolToken, decoder::Decoder, encoder::Encoder};

/// A single EVM word
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct Word(
  /// Bytes
  pub [u8; 32],
);

impl Lease<[u8]> for Word {
  #[inline]
  fn lease(&self) -> &[u8] {
    &self.0
  }
}

impl<'de> SolToken<'de> for Word {
  const IS_DYN_TOKEN: bool = false;

  #[inline]
  fn decode_from(dec: &mut Decoder<'de>) -> crate::Result<Self> {
    dec.take_word()
  }

  #[inline]
  fn head_push(&self, enc: &mut Encoder<'_>) -> crate::Result<()> {
    enc.push_words([*self])
  }

  #[inline]
  fn head_words(&self) -> usize {
    1
  }

  #[inline]
  fn tail_push(&self, _: &mut Encoder<'_>) -> crate::Result<()> {
    Ok(())
  }

  #[inline]
  fn tail_words(&self) -> usize {
    0
  }
}
