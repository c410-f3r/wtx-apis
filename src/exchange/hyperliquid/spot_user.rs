#[derive(Debug, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SpotUser {
  /// See [`ClassTransfer`]
  pub class_transfer: ClassTransfer,
}

#[derive(Debug, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ClassTransfer {
  /// Dollars without decimals
  pub usdc: u64,
  /// To perp or stop
  pub to_perp: bool,
}
