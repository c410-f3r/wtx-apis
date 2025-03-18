use wtx::misc::Vector;

/// Raw bytes wrapper
#[derive(Debug, Default, Eq, PartialEq)]
pub struct Bytes(pub Vector<u8>);

impl<T> From<T> for Bytes
where
  T: Into<Vector<u8>>,
{
  #[inline]
  fn from(data: T) -> Self {
    Bytes(data.into())
  }
}

mod serde {
  use crate::blockchain::ethereum::Bytes;
  use alloc::string::String;
  use core::fmt::{Display, Formatter};
  use serde::{
    Deserialize, Deserializer, Serialize, Serializer,
    de::{Unexpected, Visitor},
  };

  impl<'de> Deserialize<'de> for Bytes {
    #[inline]
    fn deserialize<D>(deserializer: D) -> Result<Bytes, D::Error>
    where
      D: Deserializer<'de>,
    {
      deserializer.deserialize_identifier(BytesVisitor)
    }
  }

  impl Serialize for Bytes {
    #[inline]
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
      S: Serializer,
    {
      serializer.collect_str(&HexDisplay(&self.0))
    }
  }

  struct BytesVisitor;

  impl<'de> Visitor<'de> for BytesVisitor {
    type Value = Bytes;

    #[inline]
    fn expecting(&self, formatter: &mut Formatter<'_>) -> core::fmt::Result {
      write!(formatter, "a 0x-prefixed hex-encoded vector of bytes")
    }

    #[inline]
    fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
    where
      E: serde::de::Error,
    {
      if value.len() >= 2 && value.get(..2).unwrap_or_default() == "0x" {
        Ok(Bytes(
          hex::decode(value.get(2..).unwrap_or_default()).map_err(|err| E::custom(err))?.into(),
        ))
      } else {
        Err(E::invalid_value(Unexpected::Str(value), &"0x prefix"))
      }
    }

    #[inline]
    fn visit_string<E>(self, value: String) -> Result<Self::Value, E>
    where
      E: serde::de::Error,
    {
      self.visit_str(value.as_ref())
    }
  }

  struct HexDisplay<'bytes>(&'bytes [u8]);

  impl<'bytes> Display for HexDisplay<'bytes> {
    #[inline]
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
      write!(f, "0x")?;
      for &byte in self.0.iter() {
        write!(f, "{:02x}", byte)?;
      }
      Ok(())
    }
  }
}
