#![allow(dead_code, reason = "Conditional features")]

use chrono::{Datelike, NaiveDate};
use serde::{Deserialize, de::Error};

pub(crate) fn deserialize<'de, D>(deserializer: D) -> Result<Option<NaiveDate>, D::Error>
where
  D: serde::de::Deserializer<'de>,
{
  let str = <&str>::deserialize(deserializer)?.get(..10).unwrap_or_default();
  if str.is_empty() {
    return Ok(None);
  }
  Ok(Some(
    NaiveDate::parse_from_str(str, "%Y-%m-%d").map_err(|_err| Error::custom("Invalid date"))?,
  ))
}

pub(crate) fn serialize<S>(value: &Option<NaiveDate>, serializer: S) -> Result<S::Ok, S::Error>
where
  S: serde::Serializer,
{
  match value {
    Some(elem) => serializer.collect_str(&format_args!(
      "{:04}-{:02}-{:02}",
      elem.year(),
      elem.month(),
      elem.day(),
    )),
    None => serializer.serialize_none(),
  }
}
