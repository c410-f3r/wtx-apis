/// Cancels many orders
#[derive(Debug, serde::Serialize)]
pub struct BulkCancel<'any> {
  /// See [`CancelReq`].
  pub cancels: &'any [CancelReq],
}

#[derive(Debug, serde::Serialize)]
pub struct BulkCancelCloid<'any> {
  /// See [`CancelCloidReq`].
  pub cancels: &'any [CancelCloidReq<'any>],
}

/// Cancels an order according to the fields
#[derive(Debug, serde::Deserialize, serde::Serialize)]
pub struct CancelReq {
  /// Asset
  #[serde(rename = "a", alias = "asset")]
  pub asset: u32,
  /// Order ID
  #[serde(rename = "o", alias = "oid")]
  pub oid: u64,
}

#[derive(Debug, serde::Deserialize, serde::Serialize)]
pub struct CancelCloidReq<'any> {
  pub asset: u32,
  pub cloid: &'any str,
}
