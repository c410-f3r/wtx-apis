use wtx::misc::Lease;

use crate::blockchain::ethereum::{SolToken, decoder::Decoder, encoder::Encoder};

/// A single EVM word
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct Word(
  /// Bytes
  pub [u8; 32],
);

impl Lease<[u8]> for Word {
  fn lease(&self) -> &[u8] {
    &self.0
  }
}

impl<'de> SolToken<'de> for Word {
  const IS_DYN_TOKEN: bool = false;

  fn decode_from(dec: &mut Decoder<'de>) -> crate::Result<Self> {
    dec.take_word()
  }

  fn head_push(&self, enc: &mut Encoder) -> crate::Result<()> {
    enc.push_words([*self])
  }

  fn head_words(&self) -> usize {
    1
  }

  fn tail_push(&self, _: &mut Encoder) -> crate::Result<()> {
    Ok(())
  }

  fn tail_words(&self) -> usize {
    0
  }
}
