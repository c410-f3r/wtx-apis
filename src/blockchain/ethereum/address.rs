use crate::blockchain::ethereum::{SolTy, Word};
use core::fmt::{Formatter, LowerHex};
use wtx::{
  collection::Vector,
  de::{HexEncMode, decode_hex, encode_hex},
};

/// Address
#[derive(Debug, Default, serde::Serialize)]
#[serde(transparent)]
pub struct Address(
  /// Bytes
  pub [u8; 20],
);

impl Address {
  /// From hex string
  pub fn from_hex(hex: &str) -> crate::Result<Self> {
    let mut array = [0; 20];
    let _ = decode_hex(hex.as_bytes(), &mut array)?;
    Ok(Self(array))
  }

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

impl LowerHex for Address {
  #[inline]
  fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
    let mut buffer = [0; 42];
    let Ok(hex) = encode_hex(&self.0, Some(HexEncMode::Eip55), &mut buffer) else {
      return Ok(());
    };
    f.write_str(hex.get(2..).unwrap_or_default())?;
    Ok(())
  }
}
