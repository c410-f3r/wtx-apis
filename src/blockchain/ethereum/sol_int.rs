use crate::blockchain::ethereum::{SolTy, Word};
use crypto_bigint::{Encoding as _, I256, U256};
use wtx::collections::Vector;

/// Solidity Integer
#[derive(Debug, serde::Serialize)]
#[serde(transparent)]
pub struct SolInt<T, const BITS: usize>(
  /// Integer instance
  pub T,
);

impl<T, const BITS: usize> SolTy<'_> for SolInt<T, BITS>
where
  SolInt<T, BITS>: SolIntParams,
{
  const ENCODED_SIZE: Option<usize> = Some(32);
  const PACKED_ENCODED_SIZE: Option<usize> = Some(BITS / 8);
  const SOL_NAME: &'static str = SolInt::<T, BITS>::UINT_NAME;

  type DeToken<'any> = Self;
  type Token<'any> = Word;

  #[inline]
  fn abi_encode_packed(&self, buffer: &mut Vector<u8>) -> crate::Result<()> {
    <Self as SolIntParams>::encode_packed(self, buffer)
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
    Ok(<Self as SolIntParams>::detokenize(token))
  }

  #[inline]
  fn eip712_data_word(&self, _: &mut Vector<u8>) -> crate::Result<Word> {
    Ok(<Self as SolIntParams>::tokenize(self))
  }

  #[inline]
  fn tokenize(&self) -> crate::Result<Self::Token<'_>> {
    Ok(<Self as SolIntParams>::tokenize(self))
  }

  #[inline]
  fn valid_token(token: &Self::Token<'_>) -> bool {
    let zeros = [0u8; 32];
    let slice = zeros.get(..<SolInt<T, BITS> as SolIntParams>::WORD_MSB).unwrap_or_default();
    token.0.get(..<SolInt<T, BITS> as SolIntParams>::WORD_MSB) == Some(slice)
  }
}

trait SolIntParams {
  const BITS: usize;
  const BYTES: usize = Self::BITS / 8;
  const SKIP_BYTES: usize;
  const UINT_NAME: &'static str;
  const WORD_MSB: usize = 32 - Self::BYTES;

  fn detokenize(token: Word) -> Self;

  fn encode_packed(&self, buffer: &mut Vector<u8>) -> crate::Result<()>;

  fn tokenize(&self) -> Word;
}

macro_rules! generic_impl {
  (@primitive_uint $uty:ident) => {
    #[inline]
    fn detokenize(mut token: Word) -> Self {
      token.0[Self::WORD_MSB - Self::SKIP_BYTES..Self::WORD_MSB].fill(0);
      let str = &token.0[Self::WORD_MSB - Self::SKIP_BYTES..];
      Self(<$uty>::from_be_bytes(str.try_into().unwrap()))
    }

    #[inline]
    fn encode_packed(&self, out: &mut Vector<u8>) -> crate::Result<()> {
      out.extend_from_copyable_slice(&self.0.to_be_bytes()[Self::SKIP_BYTES..])?;
      Ok(())
    }

    #[inline]
    fn tokenize(&self) -> Word {
      let mut word = Word([0; 32]);
      word.0[Self::WORD_MSB..].copy_from_slice(&self.0.to_be_bytes()[Self::SKIP_BYTES..]);
      word
    }
  };
  (@big_uint $uty:ident) => {
    #[inline]
    fn detokenize(mut token: Word) -> Self {
      token.0[..Self::SKIP_BYTES].fill(0);
      let str = &token.0[Self::WORD_MSB - Self::SKIP_BYTES..];
      Self(<$uty>::from_be_bytes(str.try_into().unwrap()))
    }

    #[inline]
    fn encode_packed(&self, out: &mut Vector<u8>) -> crate::Result<()> {
      if let Some(elem) = self.0.to_be_bytes().get(Self::SKIP_BYTES..) {
        out.extend_from_copyable_slice(elem)?;
      }
      Ok(())
    }

    #[inline]
    fn tokenize(&self) -> Word {
      let mut word = Word([0; 32]);
      if let Some(elem) = self.0.to_be_bytes().get(Self::SKIP_BYTES..) {
        word.0[Self::WORD_MSB..].copy_from_slice(elem);
      }
      word
    }
  };
}

macro_rules! uint_impl {
  (u8) => {
    generic_impl!(@primitive_uint u8);
  };
  (u16) => {
    generic_impl!(@primitive_uint u16);
  };
  (u32) => {
    generic_impl!(@primitive_uint u32);
  };
  (u64) => {
    generic_impl!(@primitive_uint u64);
  };
  (u128) => {
    generic_impl!(@primitive_uint u128);
  };
  ($t:ident) => {
    generic_impl!(@big_uint $t);
  };
}

macro_rules! overall_impl {
  ($(($n:literal, $i:ident, $u:ident)),* $(,)?) => {
    $(
      impl SolIntParams for SolInt<$u, $n> {
        const BITS: usize = $n;
        #[expect(clippy::as_conversions, reason = "compiler support")]
        const SKIP_BYTES: usize = (<$i>::BITS as usize - <Self as SolIntParams>::BITS) / 8;
        const UINT_NAME: &'static str = concat!("uint", $n);

        uint_impl!($u);
      }
    )*
  };
}

overall_impl!(
  (8, i8, u8),
  (16, i16, u16),
  (32, i32, u32),
  (64, i64, u64),
  (128, i128, u128),
  (256, I256, U256),
);
