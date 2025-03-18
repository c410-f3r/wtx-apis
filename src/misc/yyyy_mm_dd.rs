#![allow(dead_code, reason = "Conditional features")]

use crate::misc::yyyy_mm_dd_opt;
use chrono::NaiveDate;
use serde::de::Error;

pub(crate) fn deserialize<'de, D>(deserializer: D) -> Result<NaiveDate, D::Error>
where
  D: serde::de::Deserializer<'de>,
{
  yyyy_mm_dd_opt::deserialize(deserializer)?.ok_or_else(|| Error::custom("Invalid date"))
}

pub(crate) fn serialize<S>(value: &NaiveDate, serializer: S) -> Result<S::Ok, S::Error>
where
  S: serde::Serializer,
{
  yyyy_mm_dd_opt::serialize(&Some(*value), serializer)
}
