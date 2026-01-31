/// Auxiliary parameters used for signing
#[derive(Debug, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SignParams {
  /// Receive window in milliseconds.
  #[serde(skip_serializing_if = "Option::is_none")]
  pub recv_window: Option<u64>,
  /// Request timestamp in milliseconds.
  pub timestamp: u64,
}
