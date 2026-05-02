use core::{borrow::Borrow, cmp::Ordering, hash::Hasher};
use wtx::{
  collection::{ArrayStringU8, ArrayVectorU8},
  misc::Lease,
};

/// A [`Hash`] with a fixed-size byte array and a fixed-size array string.
pub type HashArrays<const B: usize, const S: usize> = Hash<[u8; B], ArrayStringU8<S>>;
/// A [`Hash`] with 32 bytes and a 44-character array string.
pub type HashArray32ArrayString44 = Hash<[u8; 32], ArrayStringU8<44>>;
/// A [`Hash`] with 32 bytes and a static string slice.
pub type HashArray32Static = Hash<[u8; 32], &'static str>;
/// A [`Hash`] with 32 bytes and no string representation.
pub type HashArray32Unit = Hash<[u8; 32], ()>;
/// A [`Hash`] with a static byte slice and a static string slice.
pub type HashStatics = Hash<&'static [u8], &'static str>;
/// A [`Hash`] with no byte representation and a 44-character array string.
pub type HashUnitArrayString44 = Hash<(), ArrayStringU8<44>>;

/// Any encoded hash and its associated bytes.
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[derive(Clone, Debug, Default)]
pub struct Hash<B, S> {
  bytes: B,
  str: S,
}

// Generic bytes with generic string

impl<B, S> Hash<B, S> {
  /// Creates a new instance from the given bytes and string.
  #[inline]
  pub const fn new(bytes: B, str: S) -> Self {
    Self { bytes, str }
  }
}

impl<B, S> Hash<B, S>
where
  B: Lease<[u8]>,
{
  /// Bytes reference
  #[inline]
  pub const fn bytes(&self) -> &B {
    &self.bytes
  }

  /// Owned bytes
  #[inline]
  pub fn into_bytes(self) -> B {
    self.bytes
  }

  /// Tries to convert to an array of 20 bytes
  #[inline]
  pub fn to_bytes_array_20(&self) -> crate::Result<[u8; 20]> {
    self.to_bytes_array()
  }

  /// Tries to convert to an array of 32 bytes
  #[inline]
  pub fn to_bytes_array_32(&self) -> crate::Result<[u8; 32]> {
    self.to_bytes_array()
  }

  fn to_bytes_array<const N: usize>(&self) -> crate::Result<[u8; N]> {
    Ok(self.bytes.lease().try_into().map_err(wtx::Error::from)?)
  }
}

impl<B, S> Hash<B, S>
where
  S: Lease<str>,
{
  /// Owned string
  #[inline]
  pub fn into_str(self) -> S {
    self.str
  }

  /// String reference
  #[inline]
  pub const fn str(&self) -> &S {
    &self.str
  }
}

// Generic bytes with specific string

impl<B> Hash<B, &'static str> {
  /// Static string reference
  #[inline]
  pub const fn static_str(&self) -> &'static str {
    self.str
  }
}

impl<B, const S: usize> Hash<B, ArrayStringU8<S>>
where
  B: Lease<[u8]>,
{
  /// Constructs an instance by base58-encoding the given bytes.
  #[cfg(feature = "bs58")]
  #[inline]
  pub fn from_base58_bytes(bytes: B) -> crate::Result<Self> {
    let str = Hash::from_base58_bytes_ref(bytes.lease())?.str;
    Ok(Self { bytes, str })
  }

  /// Constructs an instance by base64-encoding the given bytes.
  #[inline]
  pub fn from_base64_bytes(bytes: B) -> crate::Result<Self> {
    let str = Hash::from_base64_bytes_ref(bytes.lease())?.str;
    Ok(Self { bytes, str })
  }
}

// Specific bytes with generic string

impl<S> Hash<&'static [u8], S> {
  /// Static bytes reference
  #[inline]
  pub const fn static_bytes(&self) -> &'static [u8] {
    self.bytes
  }
}

impl<S, const B: usize> Hash<[u8; B], S>
where
  S: Lease<str>,
{
  /// Constructs an instance by base58-decoding the given string.
  #[cfg(feature = "bs58")]
  #[inline]
  pub fn from_base58_str(str: S) -> crate::Result<Self> {
    let bytes = Hash::from_base58_str_ref(str.lease())?.bytes;
    Ok(Self { bytes, str })
  }

  /// Constructs an instance by base64-decoding the given string.
  #[inline]
  pub fn from_base64_str(str: S) -> crate::Result<Self> {
    let bytes = Hash::from_base64_str_ref(str.lease())?.bytes;
    Ok(Self { bytes, str })
  }

  /// Constructs an instance by hex-decoding the given string.
  #[inline]
  pub fn from_hex_str(str: S) -> crate::Result<Self> {
    let bytes = Hash::from_hex_str_ref(str.lease())?.bytes;
    Ok(Self { bytes, str })
  }
}

// Only bytes

impl<B> Hash<B, ()> {
  /// Creates a bytes-only instance without an associated string.
  #[inline]
  pub fn from_bytes(bytes: B) -> Self {
    Self { bytes, str: () }
  }
}

impl<const B: usize> Hash<[u8; B], ()> {
  /// Decodes a base58 string reference into a bytes-only instance.
  #[cfg(feature = "bs58")]
  #[inline]
  pub fn from_base58_str_ref(str: &str) -> crate::Result<Self> {
    let mut buffer = ArrayVectorU8::new();
    let _ = crate::misc::decode_base58(&mut buffer, str.as_bytes())?;
    Ok(Self { bytes: buffer.into_inner()?, str: () })
  }

  /// Decodes a base64 string reference into a bytes-only instance.
  #[inline]
  pub fn from_base64_str_ref(str: &str) -> crate::Result<Self> {
    let mut buffer = ArrayVectorU8::new();
    let _ = crate::misc::decode_base64(&mut buffer, str.as_bytes())?;
    Ok(Self { bytes: buffer.into_inner()?, str: () })
  }

  /// Decodes a hex string reference into a bytes-only instance.
  #[inline]
  pub fn from_hex_str_ref(str: &str) -> crate::Result<Self> {
    let mut buffer = ArrayVectorU8::new();
    let _ = crate::misc::hex_decode(&mut buffer, str.as_bytes())?;
    Ok(Self { bytes: buffer.into_inner()?, str: () })
  }

  /// Encodes the inner bytes as a base58 string.
  #[cfg(feature = "bs58")]
  #[inline]
  pub fn to_base58_str<const N: usize>(&self) -> crate::Result<ArrayStringU8<N>> {
    let mut buffer = ArrayVectorU8::<u8, N>::default();
    let len = crate::misc::encode_base58(&mut buffer, &self.bytes)?.len();
    Ok(ArrayStringU8::from_parts(buffer.into_inner()?, len.try_into().unwrap_or_default())?)
  }

  /// Encodes the inner bytes as a base64 string.
  #[inline]
  pub fn to_base64_str<const N: usize>(&self) -> crate::Result<ArrayStringU8<N>> {
    let mut buffer = ArrayVectorU8::<u8, N>::default();
    let len = crate::misc::encode_base64(&mut buffer, &self.bytes)?.len();
    Ok(ArrayStringU8::from_parts(buffer.into_inner()?, len.try_into().unwrap_or_default())?)
  }
}

// Only string

impl<S> Hash<(), S> {
  /// Creates a string-only instance without associated bytes.
  #[inline]
  pub fn from_string(str: S) -> Self {
    Self { bytes: (), str }
  }
}

impl<const S: usize> Hash<(), ArrayStringU8<S>> {
  /// Encodes the given byte slice as a base58 string-only instance.
  #[cfg(feature = "bs58")]
  #[inline]
  pub fn from_base58_bytes_ref(bytes: &[u8]) -> crate::Result<Self> {
    let mut buffer = ArrayVectorU8::<u8, S>::default();
    let len = crate::misc::encode_base58(&mut buffer, bytes)?.len();
    let str = ArrayStringU8::from_parts(buffer.into_inner()?, len.try_into().unwrap_or_default())?;
    Ok(Self { bytes: (), str })
  }

  /// Encodes the given byte slice as a base64 string-only instance.
  #[inline]
  pub fn from_base64_bytes_ref(bytes: &[u8]) -> crate::Result<Self> {
    let mut buffer = ArrayVectorU8::<u8, S>::default();
    let len = crate::misc::encode_base64(&mut buffer, bytes)?.len();
    let str = ArrayStringU8::from_parts(buffer.into_inner()?, len.try_into().unwrap_or_default())?;
    Ok(Self { bytes: (), str })
  }
}

// Internal trait implementations

impl<B, S> Eq for Hash<B, S> where B: Lease<[u8]> {}

// Std implementations

impl<B, S> Lease<str> for Hash<B, S>
where
  S: Lease<str>,
{
  #[inline]
  fn lease(&self) -> &str {
    self.str.lease()
  }
}

impl<B, S> Lease<[u8]> for Hash<B, S>
where
  B: Lease<[u8]>,
{
  #[inline]
  fn lease(&self) -> &[u8] {
    self.bytes.lease()
  }
}

impl<B, S> Borrow<str> for Hash<B, S>
where
  S: Borrow<str>,
{
  #[inline]
  fn borrow(&self) -> &str {
    self.str.borrow()
  }
}

impl<B, S> Borrow<[u8]> for Hash<B, S>
where
  B: Borrow<[u8]>,
{
  #[inline]
  fn borrow(&self) -> &[u8] {
    self.bytes.borrow()
  }
}

impl<const B: usize> Copy for Hash<[u8; B], ()> {}

impl<B, S> core::hash::Hash for Hash<B, S>
where
  B: Lease<[u8]>,
{
  #[inline]
  fn hash<H>(&self, state: &mut H)
  where
    H: Hasher,
  {
    self.bytes().lease().hash(state);
  }
}

impl<B, S> Ord for Hash<B, S>
where
  B: Lease<[u8]>,
{
  #[inline]
  fn cmp(&self, other: &Self) -> Ordering {
    self.bytes().lease().cmp(other.bytes().lease())
  }
}

impl<B, S> PartialEq for Hash<B, S>
where
  B: Lease<[u8]>,
{
  #[inline]
  fn eq(&self, other: &Self) -> bool {
    self.bytes().lease() == other.bytes().lease()
  }
}

impl<B, S> PartialOrd for Hash<B, S>
where
  B: Lease<[u8]>,
{
  #[inline]
  fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
    Some(self.cmp(other))
  }
}
