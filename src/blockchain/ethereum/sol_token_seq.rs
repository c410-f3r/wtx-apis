use crate::blockchain::ethereum::{SolToken, decoder::Decoder, encoder::Encoder};

/// Solidity Token Sequence
pub trait SolTokenSeq<'de>: SolToken<'de> {
  /// Is a heterogenous type
  const IS_TUPLE: bool = false;

  /// Decodes external bytes into itself.
  fn decode_sequence(dec: &mut Decoder<'de>) -> crate::Result<Self>;

  /// Encodes itself into bytes.
  fn encode_sequence(&self, enc: &mut Encoder) -> crate::Result<()>;
}
