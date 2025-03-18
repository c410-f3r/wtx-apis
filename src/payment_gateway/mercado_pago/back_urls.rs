/// URLs for redirection back to the seller's site.
#[derive(Debug, serde::Deserialize, serde::Serialize)]
pub struct BackUrls<T> {
  /// Failure URL.
  pub failure: Option<T>,
  /// Pending payment URL.
  pub pending: Option<T>,
  /// Success URL.
  pub success: Option<T>,
}
