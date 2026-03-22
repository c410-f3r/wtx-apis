use core::{
  fmt::{Debug, Display, Formatter},
  ops::{Not, Shl, Shr},
  str,
};
use wtx::misc::{ArithmeticError, Usize};

/// Big-endian 256 bit integer type.
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct U256(
  // High
  u128,
  /// Low
  u128,
);

impl U256 {
  /// The value `1` represented as an `U256`.
  pub const ONE: U256 = U256(0, 1);
  /// The value `0` represented as an `U256`.
  pub const ZERO: U256 = U256(0, 0);
  /// The maximum representable `U256` value.
  pub const MAX: U256 = U256(u128::MAX, u128::MAX);

  /// New instance with the provided raw data.
  #[inline]
  pub const fn new(high: u128, low: u128) -> Self {
    Self(high, low)
  }

  /// Returns the sum, or an error on overflow.
  #[inline]
  pub fn add(&self, other: Self) -> crate::Result<Self> {
    let (rslt, overflow) = self.overflowing_add(other);
    if overflow {
      return Err(wtx::Error::from(ArithmeticError::AddOverflow).into());
    }
    Ok(rslt)
  }

  /// Returns the quotient, or an error on division by zero.
  #[inline]
  pub fn div(&self, other: Self) -> crate::Result<Self> {
    Ok(self.div_rem(other).ok_or(wtx::Error::from(ArithmeticError::DivOverflow))?.0)
  }

  /// Returns the product, or an error on overflow.
  #[inline]
  pub fn mul(&self, other: Self) -> crate::Result<Self> {
    let (rslt, overflow) = self.overflowing_mul(other);
    if overflow {
      return Err(wtx::Error::from(ArithmeticError::MulOverflow).into());
    }
    Ok(rslt)
  }

  /// Returns the difference, or an error on underflow.
  #[inline]
  pub fn sub(&self, other: Self) -> crate::Result<Self> {
    let (rslt, overflow) = self.overflowing_sub(other);
    if overflow {
      return Err(wtx::Error::from(ArithmeticError::SubOverflow).into());
    }
    Ok(rslt)
  }

  /// Returns the remainder, or an error on division by zero.
  #[inline]
  pub fn rem(&self, other: Self) -> crate::Result<Self> {
    Ok(self.div_rem(other).ok_or(wtx::Error::from(ArithmeticError::RemOverflow))?.1)
  }

  const fn bits(&self) -> u32 {
    if self.0 > 0 {
      256u32.wrapping_sub(self.0.leading_zeros())
    } else {
      128u32.wrapping_sub(self.1.leading_zeros())
    }
  }

  fn div_rem(self, rhs: Self) -> Option<(Self, Self)> {
    if rhs.is_zero() {
      return None;
    }

    let mut sub_copy = self;
    let mut shift_copy = rhs;
    let mut ret = [0u128; 2];

    let my_bits = self.bits();
    let your_bits = rhs.bits();

    // Early return in case we are dividing by a larger number than us
    if my_bits < your_bits {
      return Some((U256::ZERO, sub_copy));
    }

    // Bitwise long division
    let mut shift = my_bits.wrapping_sub(your_bits);
    shift_copy = u256_shl(shift_copy, shift);
    loop {
      if sub_copy >= shift_copy {
        let idx: usize = Usize::from(1u32.wrapping_sub(shift / 128)).into();
        if let Some(elem) = ret.get_mut(idx) {
          *elem |= 1 << (shift % 128);
        }
        sub_copy = sub_copy.wrapping_sub(shift_copy);
      }
      shift_copy = u256_shr(shift_copy, 1);
      if shift == 0 {
        break;
      }
      shift = shift.wrapping_sub(1);
    }

    let [a, b] = ret;
    Some((U256(a, b), sub_copy))
  }

  fn fmt_decimal(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
    const DIGITS: usize = 78;
    const TEN: U256 = U256(0, 10);

    let fun = || {
      let mut buf = [0u8; DIGITS];
      let mut idx = const { DIGITS - 1 };
      let mut cur = *self;
      loop {
        let digit: u8 = cur.rem(TEN).ok()?.low_u128().try_into().ok()?;
        if let Some(elem) = buf.get_mut(idx) {
          *elem = digit.wrapping_add(b'0');
        }
        cur = cur.div(TEN).ok()?;
        if cur.is_zero() {
          break;
        }
        idx = idx.wrapping_sub(1);
      }
      Some((buf, idx))
    };
    if let Some((buf, idx)) = fun() {
      let slice = buf.get(idx..).unwrap_or_default();
      // SAFETY: Numbers are ASCII
      let str = unsafe { str::from_utf8_unchecked(slice) };
      f.pad_integral(true, "", str)
    } else {
      f.write_str("N/A")
    }
  }

  const fn is_zero(&self) -> bool {
    self.0 == 0 && self.1 == 0
  }

  const fn low_u128(&self) -> u128 {
    self.1
  }

  fn mul_u64(self, rhs: u64) -> (U256, bool) {
    let mut carry: u128 = 0;
    let mut split_le = [u128_lsb(self.1), u128_msb(self.1), u128_lsb(self.0), u128_msb(self.0)];
    for word in &mut split_le {
      let (n, _) = u128::from(rhs).carrying_mul(u128::from(*word), carry);
      *word = u128_lsb(n);
      carry = u128_msb(n).into();
    }
    let [a, b, c, d] = split_le;
    let low = u128::from(a) | (u128::from(b) << 64);
    let high = u128::from(c) | (u128::from(d) << 64);
    (Self(high, low), carry != 0)
  }

  #[must_use]
  const fn overflowing_add(self, rhs: Self) -> (Self, bool) {
    let mut ret = U256::ZERO;
    let mut ret_overflow = false;
    {
      let (high, overflow) = self.0.overflowing_add(rhs.0);
      ret.0 = high;
      ret_overflow |= overflow;
    }
    let (low, overflow) = self.1.overflowing_add(rhs.1);
    ret.1 = low;
    if overflow {
      let (high, local_overflow) = ret.0.overflowing_add(1);
      ret.0 = high;
      ret_overflow |= local_overflow;
    }
    (ret, ret_overflow)
  }

  #[must_use]
  fn overflowing_mul(self, rhs: Self) -> (Self, bool) {
    let mut ret = U256::ZERO;
    let mut ret_overflow = false;
    for idx in 0..4u32 {
      let to_mul = u128_lsb(u256_shr(rhs, 64u32.wrapping_mul(idx)).low_u128());
      let (mul_res, mul_overflow) = self.mul_u64(to_mul);
      ret_overflow |= mul_overflow;
      let shift = 64u32.wrapping_mul(idx);
      if shift > 0 {
        let shifted_out = u256_shr(mul_res, 256u32.wrapping_sub(shift));
        ret_overflow |= !shifted_out.is_zero();
      }
      let shifted = u256_shl(mul_res, shift);
      let (sum, add_overflow) = ret.overflowing_add(shifted);
      ret = sum;
      ret_overflow |= add_overflow;
    }
    (ret, ret_overflow)
  }

  #[must_use]
  fn overflowing_sub(self, rhs: Self) -> (Self, bool) {
    (self.wrapping_add(!rhs).wrapping_add(Self::ONE), rhs > self)
  }

  #[must_use]
  const fn wrapping_add(self, rhs: Self) -> Self {
    self.overflowing_add(rhs).0
  }

  #[must_use]
  const fn wrapping_shl(self, rhs: u32) -> Self {
    let shift = rhs & 0b1111_1111;
    let bit_shift = shift % 128;
    let word_shift = shift >= 128;
    let mut ret = U256::ZERO;
    if word_shift {
      ret.0 = self.1 << bit_shift;
    } else {
      ret.0 = self.0 << bit_shift;
      if bit_shift > 0 {
        ret.0 = ret.0.wrapping_add(self.1.wrapping_shr(128u32.wrapping_sub(bit_shift)));
      }
      ret.1 = self.1 << bit_shift;
    }
    ret
  }

  #[must_use]
  const fn wrapping_shr(self, rhs: u32) -> Self {
    let shift = rhs & 0b1111_1111;
    let bit_shift = shift % 128;
    let word_shift = shift >= 128;
    let mut ret = U256::ZERO;
    if word_shift {
      ret.1 = self.0 >> bit_shift;
    } else {
      ret.0 = self.0 >> bit_shift;
      ret.1 = self.1 >> bit_shift;
      if bit_shift > 0 {
        ret.1 = ret.1.wrapping_add(self.0.wrapping_shl(128u32.wrapping_sub(bit_shift)));
      }
    }
    ret
  }

  #[must_use]
  fn wrapping_sub(self, rhs: Self) -> Self {
    self.overflowing_sub(rhs).0
  }
}

impl Debug for U256 {
  #[inline]
  fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
    Display::fmt(self, f)
  }
}

impl Display for U256 {
  #[inline]
  fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
    if self.is_zero() { f.pad_integral(true, "", "0") } else { self.fmt_decimal(f) }
  }
}

impl From<u64> for U256 {
  #[inline]
  fn from(n: u64) -> Self {
    U256(0, n.into())
  }
}

impl From<u128> for U256 {
  #[inline]
  fn from(n: u128) -> Self {
    U256(0, n)
  }
}

impl Not for U256 {
  type Output = Self;

  #[inline]
  fn not(self) -> Self {
    U256(!self.0, !self.1)
  }
}

impl Shl<u32> for U256 {
  type Output = Self;

  #[inline]
  fn shl(self, rhs: u32) -> U256 {
    self.wrapping_shl(rhs)
  }
}

impl Shr<u32> for U256 {
  type Output = Self;

  #[inline]
  fn shr(self, rhs: u32) -> U256 {
    self.wrapping_shr(rhs)
  }
}

#[cfg(feature = "rust_decimal")]
impl TryFrom<rust_decimal::Decimal> for U256 {
  type Error = crate::Error;

  fn try_from(from: rust_decimal::Decimal) -> Result<Self, Self::Error> {
    Ok(U256::from(u128::try_from(from).map_err(|_err| crate::Error::InvalidU256Conversion)?))
  }
}

impl TryFrom<U256> for i128 {
  type Error = crate::Error;

  #[inline]
  fn try_from(from: U256) -> Result<Self, Self::Error> {
    if from > U256::from(u128::MAX >> 1) {
      Err(crate::Error::InvalidU256Conversion)
    } else {
      Ok(<_>::try_from(from.low_u128()).map_err(|_err| crate::Error::InvalidU256Conversion)?)
    }
  }
}

impl TryFrom<U256> for u128 {
  type Error = crate::Error;

  #[inline]
  fn try_from(from: U256) -> Result<Self, Self::Error> {
    if from > U256::from(u128::MAX) {
      Err(crate::Error::InvalidU256Conversion)
    } else {
      Ok(from.low_u128())
    }
  }
}

#[expect(
  clippy::as_conversions,
  clippy::cast_possible_truncation,
  reason = "intentional operations"
)]
const fn u128_lsb(n: u128) -> u64 {
  n as u64
}

#[expect(clippy::as_conversions, reason = "intentional operation")]
const fn u128_msb(n: u128) -> u64 {
  (n >> 64) as u64
}

#[expect(clippy::arithmetic_side_effects, reason = "`n` is lesser than 256")]
fn u256_shl(u256: U256, n: u32) -> U256 {
  u256 << n
}

#[expect(clippy::arithmetic_side_effects, reason = "`n` is lesser than 256")]
fn u256_shr(u256: U256, n: u32) -> U256 {
  u256 >> n
}

#[cfg(test)]
mod tests {
  use crate::misc::U256;

  #[test]
  fn basic_arithmetic_works() {
    let max_minus_one = U256::from(u128::MAX - 1);
    let two = U256::from(2u64);
    assert_eq!(max_minus_one.mul(two).unwrap().div(two).unwrap(), max_minus_one);
    assert_eq!(two.mul(two).unwrap(), U256::from(4u64))
  }
}
