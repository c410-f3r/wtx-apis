use crate::blockchain::solana::short_vec::ShortU16Visitor;
use serde::{Deserialize, Serialize, Serializer, de::Deserializer, ser::SerializeTuple as _};

pub(crate) struct ShortU16(pub(crate) u16);

impl<'de> Deserialize<'de> for ShortU16 {
  fn deserialize<D>(deserializer: D) -> Result<ShortU16, D::Error>
  where
    D: Deserializer<'de>,
  {
    deserializer.deserialize_tuple(3, ShortU16Visitor)
  }
}

impl Serialize for ShortU16 {
  fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
  where
    S: Serializer,
  {
    let mut seq = serializer.serialize_tuple(1)?;
    let mut rem_val = self.0;
    loop {
      let mut elem: u8 = (rem_val & 0b0000_0000_0111_1111).try_into().unwrap_or_default();
      rem_val = rem_val.wrapping_shr(7);
      if rem_val == 0 {
        seq.serialize_element(&elem)?;
        break;
      }
      elem |= 0x80;
      seq.serialize_element(&elem)?;
    }
    seq.end()
  }
}
