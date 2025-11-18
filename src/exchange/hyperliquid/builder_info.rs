use wtx::collection::ArrayStringU8;

/// Those who build applications on Hyperliquid.
#[derive(Debug, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct BuilderInfo {
  /// Builder's address
  #[serde(rename = "b")]
  pub builder: ArrayStringU8<42>,
  /// Builder's fee
  #[serde(rename = "f")]
  pub fee: u64,
}
