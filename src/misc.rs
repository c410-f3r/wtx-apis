//! Utility functions and structures

mod concat_array_str;
mod hash;
#[cfg(any(feature = "mercado-pago", feature = "olist"))]
mod oauth;
mod slice_by_commas;
mod u256;
#[cfg(feature = "olist")]
pub(crate) mod yyyy_mm_dd;
#[cfg(feature = "olist")]
pub(crate) mod yyyy_mm_dd_opt;

#[cfg(feature = "base64")]
use base64::{Engine as _, engine::general_purpose::STANDARD};
pub use concat_array_str::ConcatArrayStr;
use core::str;
pub use hash::*;
#[cfg(any(feature = "mercado-pago", feature = "olist"))]
pub use oauth::*;
#[cfg(feature = "serde")]
use serde::{Deserialize, Deserializer, Serialize, Serializer, de::IntoDeserializer as _};
pub use slice_by_commas::SliceByCommas;
pub use u256::U256;
use wtx::{
  collection::{ArrayStringU8, CapacityUpperBound, LinearStorageLen, Truncate, TryExtend},
  misc::{Lease as _, LeaseMut},
};

const MAX_ASSET_ABBR_LEN: usize = 10;
const MAX_NUMBER_LEN: usize = 31;

/// Maximum asset abbreviation like BTC.
pub type MaxAssetAbbr = ArrayStringU8<MAX_ASSET_ABBR_LEN>;
/// Maximum asset name like Bitcoin.
pub type MaxAssetName = ArrayStringU8<36>;
/// Maximum string representation of a number.
pub type MaxNumberStr = ArrayStringU8<MAX_NUMBER_LEN>;
/// Maximum pair abbreviation like ETH-BTC
pub type MaxPairAbbr = ArrayStringU8<{ 2 * MAX_ASSET_ABBR_LEN + 1 }>;

_create_blockchain_constants!(
  pub address_hash: MaxAddressHash = 32,
  pub address_hash_str: MaxAddressHashStr = 66,
  pub block_hash: MaxBlockHash = 32,
  pub block_hash_str: MaxBlockHashStr = 67,
  pub signature_hash: MaxSignatureHash = 64,
  pub signature_hash_str: MaxSignatureHashStr = 90,
  pub transaction_hash: MaxTransactionHash = 64,
  pub transaction_hash_str: MaxTransactionHashStr = 90
);

/// Decodes to Base58
#[cfg(feature = "bs58")]
pub fn decode_base58<'buffer, B, L>(
  buffer: &'buffer mut B,
  bytes: &[u8],
) -> crate::Result<&'buffer mut [u8]>
where
  B: CapacityUpperBound + LeaseMut<[u8]> + Truncate<L> + TryExtend<(u8, usize)>,
  L: LinearStorageLen,
{
  let max_decoded_len = (bytes.len() * 80) / 100;
  decode_into_buffer(buffer, max_decoded_len.min(B::CAPACITY_UPPER_BOUND), |slice| {
    bs58::decode(bytes).onto(slice).map_err(|_| crate::Error::Bs58Error)
  })
}

/// Decodes to Base64
#[cfg(feature = "base64")]
pub fn decode_base64<'buffer, B, L>(
  buffer: &'buffer mut B,
  bytes: &[u8],
) -> crate::Result<&'buffer mut [u8]>
where
  B: CapacityUpperBound + LeaseMut<[u8]> + Truncate<L> + TryExtend<(u8, usize)>,
  L: LinearStorageLen,
{
  let max_decoded_len = base64::decoded_len_estimate(bytes.len());
  decode_into_buffer(buffer, max_decoded_len.min(B::CAPACITY_UPPER_BOUND), |slice| {
    STANDARD.decode_slice(bytes, slice).map_err(|err| wtx::Error::from(err).into())
  })
}

/// Decodes to Hex
#[inline]
pub fn decode_hex<'buffer, B, L>(
  buffer: &'buffer mut B,
  bytes: &[u8],
) -> crate::Result<&'buffer mut [u8]>
where
  B: CapacityUpperBound + LeaseMut<[u8]> + Truncate<L> + TryExtend<(u8, usize)>,
  L: LinearStorageLen,
{
  let max_decoded_len = bytes.len() / 2;
  decode_into_buffer(buffer, max_decoded_len.min(B::CAPACITY_UPPER_BOUND), |slice| {
    Ok(wtx::codec::decode_hex(bytes, slice)?.len())
  })
}

/// Deserializes an Base58 string as an array of bytes.
#[cfg(all(feature = "bs58", feature = "serde"))]
#[inline]
pub fn deserialize_array_from_base58<'de, D, const N: usize>(
  deserializer: D,
) -> Result<[u8; N], D::Error>
where
  D: Deserializer<'de>,
{
  let s: &str = Deserialize::deserialize(deserializer)?;
  let mut array = [0; N];
  bs58::decode(s)
    .onto(&mut array)
    .ok()
    .and_then(|len| {
      if len != N {
        return None;
      }
      Some(())
    })
    .ok_or_else(|| serde::de::Error::custom("Could not deserialize base58 hash string"))?;
  Ok(array)
}

/// Deserializes an arbitrary type from a string.
#[cfg(feature = "serde")]
#[inline]
pub fn deserialize_from_str<'de, D, T>(deserializer: D) -> Result<T, D::Error>
where
  T: str::FromStr,
  T::Err: core::fmt::Display,
  D: Deserializer<'de>,
{
  let s: &str = Deserialize::deserialize(deserializer)?;
  T::from_str(s).map_err(serde::de::Error::custom)
}

/// Deserializes an arbitrary type ignoring its contents.
#[cfg(feature = "serde")]
#[inline]
pub fn deserialize_ignore_any<'de, D, T>(deserializer: D) -> Result<T, D::Error>
where
  D: Deserializer<'de>,
  T: Default,
{
  serde::de::IgnoredAny::deserialize(deserializer).map(|_| T::default())
}

/// Deserializes an arbitrary type from an optional string.
///
/// If the deserialized string is empty, then returns `None`.
#[cfg(feature = "serde")]
#[inline]
pub fn deserialize_opt_considering_empty_str<'de, D, T>(
  deserializer: D,
) -> Result<Option<T>, D::Error>
where
  D: Deserializer<'de>,
  T: Deserialize<'de>,
{
  match <Option<&str>>::deserialize(deserializer)? {
    None | Some("") => Ok(None),
    Some(s) => T::deserialize(s.into_deserializer()).map(Some),
  }
}

/// Encodes to Base58
#[cfg(feature = "bs58")]
pub fn encode_base58<'buffer, B, L>(
  buffer: &'buffer mut B,
  bytes: &[u8],
) -> crate::Result<&'buffer str>
where
  B: CapacityUpperBound + LeaseMut<[u8]> + Truncate<L> + TryExtend<(u8, usize)>,
  L: LinearStorageLen,
{
  let max_encoded_len = ((bytes.len() * 140) / 100).wrapping_add(1);
  encode_into_buffer(buffer, max_encoded_len.min(B::CAPACITY_UPPER_BOUND), |slice| {
    bs58::encode(bytes).onto(slice).map_err(|_err| crate::Error::Bs58Error)
  })
}

/// Encodes to Base64
#[cfg(feature = "base64")]
pub fn encode_base64<'buffer, B, L>(
  buffer: &'buffer mut B,
  bytes: &[u8],
) -> crate::Result<&'buffer str>
where
  B: CapacityUpperBound + LeaseMut<[u8]> + Truncate<L> + TryExtend<(u8, usize)>,
  L: LinearStorageLen,
{
  let max_encoded_len = base64::encoded_len(bytes.len(), true).unwrap_or_default();
  encode_into_buffer(buffer, max_encoded_len.min(B::CAPACITY_UPPER_BOUND), |slice| {
    Ok(STANDARD.encode_slice(bytes, slice).map_err(wtx::Error::from)?)
  })
}

/// Encodes to Hex
#[inline]
pub fn encode_hex<'buffer, B, L>(
  buffer: &'buffer mut B,
  bytes: &[u8],
) -> crate::Result<&'buffer str>
where
  B: CapacityUpperBound + LeaseMut<[u8]> + Truncate<L> + TryExtend<(u8, usize)>,
  L: LinearStorageLen,
{
  let max_encoded_len = bytes.len().checked_mul(2).unwrap_or_default();
  encode_into_buffer(buffer, max_encoded_len.min(B::CAPACITY_UPPER_BOUND), |slice| {
    Ok(wtx::codec::encode_hex(bytes, None, slice)?.len())
  })
}

/// Serializes an arbitrary type as a tuple
#[cfg(feature = "serde")]
#[inline]
pub fn serialize_as_tuple<T, S>(field: T, serializer: S) -> Result<S::Ok, S::Error>
where
  T: Serialize,
  S: Serializer,
{
  (field,).serialize(serializer)
}

#[cfg(all(feature = "hyperliquid", feature = "serde"))]
pub(crate) fn _serialize_hex<S, T>(val: T, s: S) -> Result<S::Ok, S::Error>
where
  T: core::fmt::LowerHex,
  S: Serializer,
{
  s.collect_str(&format_args!("0x{val:x}"))
}

#[inline]
fn decode_into_buffer<B, L>(
  buffer: &mut B,
  max_decoded_len: usize,
  cb: impl FnOnce(&mut [u8]) -> crate::Result<usize>,
) -> crate::Result<&mut [u8]>
where
  B: LeaseMut<[u8]> + Truncate<L> + TryExtend<(u8, usize)>,
  L: LinearStorageLen,
{
  let prev = buffer.lease().len();
  buffer.try_extend((0, max_decoded_len))?;
  let slice = buffer.lease_mut().get_mut(prev..).unwrap_or_default();
  let len = cb(slice)?;
  buffer.truncate(L::from_usize(prev.wrapping_add(len))?);
  Ok(buffer.lease_mut().get_mut(prev..).unwrap_or_default())
}

#[inline]
fn encode_into_buffer<B, L>(
  buffer: &mut B,
  max_encoded_len: usize,
  cb: impl FnOnce(&mut [u8]) -> crate::Result<usize>,
) -> crate::Result<&str>
where
  B: LeaseMut<[u8]> + Truncate<L> + TryExtend<(u8, usize)>,
  L: LinearStorageLen,
{
  let prev = buffer.lease().len();
  buffer.try_extend((0, max_encoded_len))?;
  let slice = buffer.lease_mut().get_mut(prev..).unwrap_or_default();
  let len = cb(slice)?;
  buffer.truncate(L::from_usize(prev.wrapping_add(len))?);
  // SAFETY: calling functions produce valid UTF-8
  Ok(unsafe { str::from_utf8_unchecked(buffer.lease_mut().get_mut(prev..).unwrap_or_default()) })
}
