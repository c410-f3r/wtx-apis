use crate::blockchain::ethereum::{SolTy, Word};
use wtx::collection::Vector;

/// Address
#[derive(Debug, Default, serde::Serialize)]
#[serde(transparent)]
pub struct Address(
  /// Bytes
  pub [u8; 20],
);

impl Address {
  pub(crate) const fn from_word(word: Word) -> Self {
    let [.., a, b, c, d, e, f, g, h, i, j, k, l, m, n, o, p, q, r, s, t] = word.0;
    Self([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o, p, q, r, s, t])
  }

  pub(crate) fn into_word(&self) -> Word {
    let mut word = [0; 32];
    word[12..].copy_from_slice(&self.0);
    Word(word)
  }
}

impl<'de> SolTy<'de> for Address {
  const ENCODED_SIZE: Option<usize> = Some(32);
  const PACKED_ENCODED_SIZE: Option<usize> = Some(20);
  const SOL_NAME: &'static str = "address";

  type DeToken<'any> = Self;
  type Token<'any> = Word;

  fn abi_encode_packed(&self, buffer: &mut Vector<u8>) -> crate::Result<()> {
    buffer.extend_from_copyable_slice(&self.0)?;
    Ok(())
  }

  fn abi_encoded_size(&self) -> usize {
    const { Self::ENCODED_SIZE.unwrap() }
  }

  fn abi_packed_encoded_size(&self) -> usize {
    const { Self::PACKED_ENCODED_SIZE.unwrap() }
  }

  fn detokenize(token: Self::Token<'_>) -> crate::Result<Self> {
    Ok(Address::from_word(token))
  }

  fn eip712_data_word(&self, _: &mut Vector<u8>) -> crate::Result<Word> {
    self.tokenize()
  }

  fn tokenize(&self) -> crate::Result<Self::Token<'_>> {
    Ok(self.into_word())
  }

  fn valid_token(token: &Self::Token<'_>) -> bool {
    &token.0[..12] == &[0; 12]
  }
}
