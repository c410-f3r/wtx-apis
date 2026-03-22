use crate::blockchain::solana::{misc::sub_slice, program::LenBounds};
use core::ops::Range;
use wtx::collection::Vector;

/// Trait for types that can be packed into or unpacked from byte buffers.
pub trait PackData
where
  Self: Sized,
{
  /// Represents on-chain length of a structure.
  ///
  /// Depending on the implementation, can or can not contain pads.
  const LEN_BOUNDS: LenBounds;

  /// Returns the actual length of the packed data.
  fn len(&self) -> usize {
    const {
      if let Some(max) = Self::LEN_BOUNDS.max() {
        if Self::LEN_BOUNDS.min() == max {
          max
        } else {
          panic!("Invalid default `dyn_len` method");
        }
      } else {
        panic!("Invalid default `dyn_len` method");
      }
    }
  }

  /// Serializes the structure into the provided byte vector.
  fn pack_data(&self, buffer: &mut Vector<u8>) -> crate::Result<()>;

  /// Deserializes the structure from the given byte slice.
  fn unpack_data(bytes: &[u8]) -> crate::Result<Self>;
}

impl PackData for () {
  const LEN_BOUNDS: LenBounds = LenBounds::new(0, None);

  fn pack_data(&self, _: &mut Vector<u8>) -> crate::Result<()> {
    Ok(())
  }

  fn unpack_data(_: &[u8]) -> crate::Result<Self> {
    Ok(())
  }
}

impl<A, B> PackData for (A, B)
where
  A: PackData,
  B: PackData,
{
  const LEN_BOUNDS: LenBounds = {
    let min = A::LEN_BOUNDS.min() + B::LEN_BOUNDS.min();
    let max = match [A::LEN_BOUNDS.max(), B::LEN_BOUNDS.max()] {
      [None, None] => None,
      [None, Some(el)] | [Some(el), None] => Some(el),
      [Some(a), Some(b)] => Some(a + b),
    };
    LenBounds::new(min, max)
  };

  fn pack_data(&self, buffer: &mut Vector<u8>) -> crate::Result<()> {
    self.0.pack_data(buffer)?;
    self.1.pack_data(buffer)?;
    Ok(())
  }

  fn unpack_data(bytes: &[u8]) -> crate::Result<Self> {
    let a = A::unpack_data(bytes)?;
    let b = B::unpack_data(sub_slice(bytes, a.len()..))?;
    Ok((a, b))
  }
}

impl<T> PackData for Option<T>
where
  T: PackData,
{
  const LEN_BOUNDS: LenBounds = T::LEN_BOUNDS;

  fn pack_data(&self, buffer: &mut Vector<u8>) -> crate::Result<()> {
    if let Some(el) = self {
      1u8.pack_data(buffer)?;
      el.pack_data(buffer)?;
    } else {
      0u8.pack_data(buffer)?;
    }
    Ok(())
  }

  fn unpack_data(bytes: &[u8]) -> crate::Result<Self> {
    match bytes {
      [0] => Ok(None),
      [1, rest @ ..] => {
        Ok(Some(T::unpack_data(sub_slice(rest, 0..Self::LEN_BOUNDS.max_or_min()))?))
      }
      _ => Err(crate::Error::SolanaInvalidAccountData),
    }
  }
}

impl<T> PackData for Range<T>
where
  T: PackData,
{
  const LEN_BOUNDS: LenBounds = T::LEN_BOUNDS.saturating_add(&T::LEN_BOUNDS);

  fn pack_data(&self, buffer: &mut Vector<u8>) -> crate::Result<()> {
    let Range { start, end } = self;
    start.pack_data(buffer)?;
    end.pack_data(buffer)?;
    Ok(())
  }

  fn unpack_data(bytes: &[u8]) -> crate::Result<Self> {
    let start = T::unpack_data(bytes)?;
    let end = T::unpack_data(sub_slice(bytes, start.len()..))?;
    Ok(Self { start, end })
  }
}
