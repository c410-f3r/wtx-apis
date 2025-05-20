#![allow(dead_code, reason = "Conditional features")]

use serde::{Deserialize as _, de::Error};
use wtx::calendar::{Date, TimeToken};

static TOKENS: &[TimeToken] = &[
  TimeToken::FourDigitYear,
  TimeToken::Dash,
  TimeToken::TwoDigitMonth,
  TimeToken::Dash,
  TimeToken::TwoDigitDay,
];

pub(crate) fn deserialize<'de, D>(deserializer: D) -> Result<Option<Date>, D::Error>
where
  D: serde::de::Deserializer<'de>,
{
  let str = <&str>::deserialize(deserializer)?.get(..10).unwrap_or_default();
  if str.is_empty() {
    return Ok(None);
  }
  Ok(Some(
    Date::parse(str.as_bytes(), TOKENS.iter().copied())
      .map_err(|_err| Error::custom("Invalid date"))?,
  ))
}

#[expect(clippy::ref_option, reason = "serde's signature")]
#[expect(clippy::trivially_copy_pass_by_ref, reason = "serde's signature")]
pub(crate) fn serialize<S>(value: &Option<Date>, serializer: S) -> Result<S::Ok, S::Error>
where
  S: serde::Serializer,
{
  match value {
    Some(elem) => serializer.collect_str(&format_args!(
      "{:04}-{:02}-{:02}",
      elem.year().num(),
      elem.month().num(),
      elem.day().num(),
    )),
    None => serializer.serialize_none(),
  }
}
