/// Person Type
#[derive(Clone, Copy, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
pub enum PersonTy {
  /// Foreign
  #[serde(rename = "E")]
  Foreign,
  /// Foreign within country
  #[serde(rename = "X")]
  ForeignWithinCountry,
  /// Natural
  #[serde(rename = "F")]
  Natural,
  /// Juridical
  #[serde(rename = "J")]
  Juridical,
}
