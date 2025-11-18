use crate::blockchain::ethereum::{decoder::Decoder, encoder::Encoder};

/// Solidity Token
pub trait SolToken<'de>: Sized {
  /// Is the implementation is known at compile-time.
  const IS_DYN_TOKEN: bool;

  /// Decodes external bytes into itself.
  fn decode_from(dec: &mut Decoder<'de>) -> crate::Result<Self>;

  /// Pushes static elements
  fn head_push(&self, enc: &mut Encoder) -> crate::Result<()>;

  /// The amount of static elements
  fn head_words(&self) -> usize;

  /// Pushes dynamic elements
  fn tail_push(&self, enc: &mut Encoder) -> crate::Result<()>;

  /// The amount of dynamic elements
  fn tail_words(&self) -> usize;

  /// Utility function that sums all words.
  #[inline]
  fn total_words(&self) -> usize {
    self.head_words().wrapping_add(self.tail_words())
  }
}
