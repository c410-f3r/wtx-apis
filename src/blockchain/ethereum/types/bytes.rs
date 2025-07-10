use wtx::collection::Vector;

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
  use core::fmt::Formatter;
  use serde::{Deserialize, Deserializer, Serialize, Serializer, de::Visitor};
  use wtx::{
    collection::{IndexedStorageMut, Vector},
    de::{HexDisplay, decode_hex_to_slice},
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
      serializer.collect_str(&HexDisplay::<true>(&self.0))
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
      let mut vector = Vector::from_vec(alloc::vec![0; value.len() / 2]);
      let len = decode_hex_to_slice::<true>(value.as_bytes(), &mut vector)
        .map_err(|err| E::custom(err))?
        .len();
      vector.truncate(len);
      Ok(Bytes(vector))
    }

    #[inline]
    fn visit_string<E>(self, value: String) -> Result<Self::Value, E>
    where
      E: serde::de::Error,
    {
      self.visit_str(value.as_ref())
    }
  }
}
