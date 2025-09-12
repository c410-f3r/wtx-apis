/// Generic entity that is associated by an ID
#[derive(Debug, serde::Deserialize, serde::Serialize)]
pub struct AssociatedEntityId {
  /// Id
  #[serde(skip_serializing_if = "Option::is_none")]
  pub id: Option<u64>,
}

/// Generic entity that is associated by an ID and name
#[derive(Debug, serde::Deserialize, serde::Serialize)]
pub struct AssociatedEntityIdName<S> {
  /// Id
  #[serde(skip_serializing_if = "Option::is_none")]
  pub id: Option<u32>,
  /// Name
  #[serde(skip_serializing_if = "Option::is_none")]
  pub nome: Option<S>,
}
