use crate::blockchain::ethereum::{SolTy, Word};
use core::fmt::{Formatter, LowerHex};
use wtx::{
  codec::{eip55_encode, hex_decode},
  collections::Vector,
};

/// Address
#[derive(Clone, Debug, Default, serde::Serialize)]
#[serde(transparent)]
pub struct Address(
  /// Bytes
  pub [u8; 20],
);

impl Address {
  /// From hex string
  #[inline]
  pub fn from_hex(hex: &str) -> crate::Result<Self> {
    let mut array = [0; 20];
    let _ = hex_decode(hex.as_bytes(), &mut array).map_err(wtx::Error::from)?;
    Ok(Self(array))
  }

  pub(crate) const fn from_word(word: Word) -> Self {
    let [
      ..,
      b0,
      b1,
      b2,
      b3,
      b4,
      b5,
      b6,
      b7,
      b8,
      b9,
      b10,
      b11,
      b12,
      b13,
      b14,
      b15,
      b16,
      b17,
      b18,
      b19,
    ] = word.0;
    Self([b0, b1, b2, b3, b4, b5, b6, b7, b8, b9, b10, b11, b12, b13, b14, b15, b16, b17, b18, b19])
  }

  pub(crate) fn into_word(self) -> Word {
    let mut word = [0; 32];
    word[12..].copy_from_slice(&self.0);
    Word(word)
  }
}

impl SolTy<'_> for Address {
  const ENCODED_SIZE: Option<usize> = Some(32);
  const PACKED_ENCODED_SIZE: Option<usize> = Some(20);
  const SOL_NAME: &'static str = "address";

  type DeToken<'any> = Self;
  type Token<'any> = Word;

  #[inline]
  fn abi_encode_packed(&self, buffer: &mut Vector<u8>) -> crate::Result<()> {
    buffer.extend_from_copyable_slice(&self.0)?;
    Ok(())
  }

  #[inline]
  fn abi_encoded_size(&self) -> usize {
    const { Self::ENCODED_SIZE.unwrap() }
  }

  #[inline]
  fn abi_packed_encoded_size(&self) -> usize {
    const { Self::PACKED_ENCODED_SIZE.unwrap() }
  }

  #[inline]
  fn detokenize(token: Self::Token<'_>) -> crate::Result<Self> {
    Ok(Address::from_word(token))
  }

  #[inline]
  fn eip712_data_word(&self, _: &mut Vector<u8>) -> crate::Result<Word> {
    self.tokenize()
  }

  #[inline]
  fn tokenize(&self) -> crate::Result<Self::Token<'_>> {
    Ok(self.clone().into_word())
  }

  #[inline]
  fn valid_token(token: &Self::Token<'_>) -> bool {
    token.0[..12] == [0; 12]
  }
}

impl LowerHex for Address {
  #[inline]
  fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
    let mut buffer = [0; 42];
    let Ok(hex) = eip55_encode(&self.0, &mut buffer) else {
      return Ok(());
    };
    f.write_str(hex.get(2..).unwrap_or_default())?;
    Ok(())
  }
}
