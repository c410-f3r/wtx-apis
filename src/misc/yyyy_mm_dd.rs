#![allow(dead_code, reason = "Conditional features")]

use crate::misc::yyyy_mm_dd_opt;
use serde::de::Error;
use wtx::calendar::Date;

pub(crate) fn deserialize<'de, D>(deserializer: D) -> Result<Date, D::Error>
where
  D: serde::de::Deserializer<'de>,
{
  yyyy_mm_dd_opt::deserialize(deserializer)?.ok_or_else(|| Error::custom("Invalid date"))
}

#[expect(clippy::trivially_copy_pass_by_ref, reason = "serde's signature")]
pub(crate) fn serialize<S>(value: &Date, serializer: S) -> Result<S::Ok, S::Error>
where
  S: serde::Serializer,
{
  yyyy_mm_dd_opt::serialize(&Some(*value), serializer)
}
