#[derive(Debug, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
/// Package format
pub enum PackageFormat {
  /// Boc
  Box,
}
