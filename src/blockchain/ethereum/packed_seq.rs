use crate::blockchain::ethereum::{SolToken, decoder::Decoder, encoder::Encoder, misc::words_for};

/// Packed Sequence
///
/// A collection of variable length.
#[derive(Debug, PartialEq)]
pub struct PackedSeq<'any>(
  /// Bytes
  pub &'any [u8],
);

impl<'de, 'local_de> SolToken<'de> for PackedSeq<'local_de>
where
  'de: 'local_de,
{
  const IS_DYN_TOKEN: bool = true;

  fn decode_from(dec: &mut Decoder<'de>) -> crate::Result<Self> {
    let mut child = dec.take_tail()?;
    let len = child.take_tail_idx()?;
    Ok(Self(child.tail_bytes(len)?))
  }

  fn head_push(&self, enc: &mut Encoder) -> crate::Result<()> {
    enc.push_last_tail_idx()?;
    Ok(())
  }

  fn head_words(&self) -> usize {
    1
  }

  fn tail_push(&self, enc: &mut Encoder) -> crate::Result<()> {
    enc.push_packed_sequence(&self.0)
  }

  fn tail_words(&self) -> usize {
    words_for(&self.0).wrapping_add(1)
  }
}
