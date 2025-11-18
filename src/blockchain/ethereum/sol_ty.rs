use crate::blockchain::ethereum::{
  PackedSeq, SolToken, SolTokenSeq, Word,
  encoder::Encoder,
  misc::{has_only_zeros, keccak256, padded_len},
};
use wtx::collection::{ArrayWrapper, Vector};

/// Solidity Type
pub trait SolTy<'de>: Sized {
  /// Static version of [`SolTy::abi_encoded_size`] if the len is known at compile-time.
  const ENCODED_SIZE: Option<usize>;
  /// If the implementation is known at compile-time.
  const IS_DYN_TY: bool = Self::ENCODED_SIZE.is_none();
  /// Static version of [`SolTy::abi_packed_encoded_size`] if the len is known at compile-time.
  const PACKED_ENCODED_SIZE: Option<usize>;
  /// Solidity name
  const SOL_NAME: &'static str;

  /// Encoded Token
  type Token<'any>: SolToken<'any>;
  /// Decoded Token
  type DeToken<'any>;

  /// Encodes itself into a standard byte array.
  fn abi_encode(&self, enc: &mut Encoder) -> crate::Result<()> {
    let token = self.tokenize()?;
    (token,).encode_sequence(enc)?;
    Ok(())
  }

  /// Non-standard Packed Mode ABI encoding.
  fn abi_encode_packed(&self, babi_encodeuffer: &mut Vector<u8>) -> crate::Result<()>;

  /// Encoded size returned by [`SolTy::abi_encoded_size`].
  fn abi_encoded_size(&self) -> usize;

  /// Encoded size returned by [`SolTy::abi_encode_packed`].
  fn abi_packed_encoded_size(&self) -> usize;

  /// Decodes a token into itself
  fn detokenize(token: Self::Token<'de>) -> crate::Result<Self::DeToken<'de>>;

  /// Encodes itself according to the EIP-712 rules.
  fn eip712_data_word(&self, buffer: &mut Vector<u8>) -> crate::Result<Word>;

  /// Encodes itself into a token
  fn tokenize(&self) -> crate::Result<Self::Token<'_>>;

  /// If the associated token is valid
  fn valid_token(token: &Self::Token<'_>) -> bool;
}

impl<'de, T> SolTy<'de> for &'de T
where
  T: SolTy<'de>,
{
  const ENCODED_SIZE: Option<usize> = T::ENCODED_SIZE;
  const PACKED_ENCODED_SIZE: Option<usize> = T::PACKED_ENCODED_SIZE;
  const SOL_NAME: &'static str = T::SOL_NAME;

  type DeToken<'any> = T::DeToken<'de>;
  type Token<'any> = T::Token<'any>;

  fn abi_encode_packed(&self, buffer: &mut Vector<u8>) -> crate::Result<()> {
    (**self).abi_encode_packed(buffer)
  }

  fn abi_encoded_size(&self) -> usize {
    (**self).abi_encoded_size()
  }

  fn abi_packed_encoded_size(&self) -> usize {
    (**self).abi_packed_encoded_size()
  }

  fn detokenize(token: Self::Token<'de>) -> crate::Result<Self::DeToken<'de>> {
    T::detokenize(token)
  }

  fn eip712_data_word(&self, buffer: &mut Vector<u8>) -> crate::Result<Word> {
    (**self).eip712_data_word(buffer)
  }

  fn tokenize(&self) -> crate::Result<Self::Token<'_>> {
    (**self).tokenize()
  }

  fn valid_token(token: &Self::Token<'_>) -> bool {
    T::valid_token(token)
  }
}

impl<'de, T> SolTy<'de> for &'de mut T
where
  T: SolTy<'de>,
{
  const ENCODED_SIZE: Option<usize> = T::ENCODED_SIZE;
  const PACKED_ENCODED_SIZE: Option<usize> = T::PACKED_ENCODED_SIZE;
  const SOL_NAME: &'static str = T::SOL_NAME;

  type DeToken<'any> = T::DeToken<'de>;
  type Token<'any> = T::Token<'any>;

  fn abi_encode_packed(&self, buffer: &mut Vector<u8>) -> crate::Result<()> {
    (**self).abi_encode_packed(buffer)
  }

  fn abi_encoded_size(&self) -> usize {
    (**self).abi_encoded_size()
  }

  fn abi_packed_encoded_size(&self) -> usize {
    (**self).abi_packed_encoded_size()
  }

  fn detokenize(token: Self::Token<'de>) -> crate::Result<Self::DeToken<'de>> {
    T::detokenize(token)
  }

  fn eip712_data_word(&self, buffer: &mut Vector<u8>) -> crate::Result<Word> {
    (**self).eip712_data_word(buffer)
  }

  fn tokenize(&self) -> crate::Result<Self::Token<'_>> {
    (**self).tokenize()
  }

  fn valid_token(token: &Self::Token<'_>) -> bool {
    T::valid_token(token)
  }
}

impl<'de> SolTy<'de> for &'de [u8] {
  const ENCODED_SIZE: Option<usize> = Some(32);
  const PACKED_ENCODED_SIZE: Option<usize> = Some(20);
  const SOL_NAME: &'static str = "address";

  type DeToken<'any> = Self;
  type Token<'any> = PackedSeq<'any>;

  fn abi_encode_packed(&self, buffer: &mut Vector<u8>) -> crate::Result<()> {
    buffer.extend_from_copyable_slice(self)?;
    Ok(())
  }

  fn abi_encoded_size(&self) -> usize {
    if self.is_empty() { 64 } else { padded_len(self).wrapping_add(64) }
  }

  fn abi_packed_encoded_size(&self) -> usize {
    self.len()
  }

  fn detokenize(token: Self::Token<'de>) -> crate::Result<Self::DeToken<'de>> {
    Ok(token.0)
  }

  fn eip712_data_word(&self, buffer: &mut Vector<u8>) -> crate::Result<Word> {
    self.abi_encode_packed(buffer)?;
    let array = keccak256([buffer]);
    Ok(Word(array))
  }

  fn tokenize(&self) -> crate::Result<Self::Token<'_>> {
    Ok(PackedSeq(self))
  }

  fn valid_token(_: &Self::Token<'_>) -> bool {
    true
  }
}

impl<'de, const N: usize> SolTy<'de> for ArrayWrapper<u8, N>
where
  ByteCount<N>: SupportedFixedBytes,
{
  const ENCODED_SIZE: Option<usize> = Some(32);
  const PACKED_ENCODED_SIZE: Option<usize> = Some(N);
  const SOL_NAME: &'static str = <ByteCount<N>>::NAME;

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

  fn detokenize(token: Self::Token<'de>) -> crate::Result<Self::DeToken<'de>> {
    Ok(Self(token.0.as_ref().try_into().map_err(wtx::Error::from)?))
  }

  fn eip712_data_word(&self, _: &mut Vector<u8>) -> crate::Result<Word> {
    self.tokenize()
  }

  fn tokenize(&self) -> crate::Result<Self::Token<'_>> {
    let mut word = Word::default();
    word.0.get_mut(..N).ok_or(crate::Error::BytesAreGreaterThanWord)?.copy_from_slice(&self.0);
    Ok(word)
  }

  fn valid_token(token: &Self::Token<'_>) -> bool {
    has_only_zeros(&token.0[N..])
  }
}

struct ByteCount<const N: usize>;

trait SupportedFixedBytes {
  const NAME: &'static str;
}

macro_rules! supported_fixed_bytes {
  ($($n:literal),+) => {$(
    impl SupportedFixedBytes for ByteCount<$n> {
      const NAME: &'static str = concat!("bytes", $n);
    }
  )+};
}

supported_fixed_bytes!(
  1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26,
  27, 28, 29, 30, 31, 32
);
