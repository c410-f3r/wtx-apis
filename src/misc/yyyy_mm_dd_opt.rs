#![allow(dead_code, reason = "Conditional features")]

use serde::{Deserialize as _, de::Error};
use wtx::time::DateTime;

pub(crate) fn deserialize<'de, D>(deserializer: D) -> Result<Option<DateTime>, D::Error>
where
  D: serde::de::Deserializer<'de>,
{
  let str = <&str>::deserialize(deserializer)?.get(..10).unwrap_or_default();
  if str.is_empty() {
    return Ok(None);
  }
  Ok(Some(
    DateTime::parse(str.as_bytes(), b"%Y-%m-%d").map_err(|_err| Error::custom("Invalid date"))?,
  ))
}

#[expect(clippy::ref_option, reason = "serde's signature")]
pub(crate) fn serialize<S>(value: &Option<DateTime>, serializer: S) -> Result<S::Ok, S::Error>
where
  S: serde::Serializer,
{
  match value {
    Some(elem) => serializer.collect_str(&format_args!(
      "{:04}-{:02}-{:02}",
      elem.date().year().num(),
      elem.date().month().num(),
      elem.date().day().num(),
    )),
    None => serializer.serialize_none(),
  }
}
