/// Vault response
#[derive(Debug, serde::Deserialize)]
pub struct VaultResponse<T> {
  /// data
  pub data: T,
}
