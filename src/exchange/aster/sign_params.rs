/// Auxiliary parameters used for centralized signing
#[derive(Debug, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CexSignParams {
  /// Receive window in milliseconds.
  #[serde(skip_serializing_if = "Option::is_none")]
  pub recv_window: Option<u64>,
}
