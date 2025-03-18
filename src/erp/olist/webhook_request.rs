/// Basic structure for Webhook requests
#[derive(Debug, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct WebhookRequest<D, S> {
  /// Registration
  pub cnpj: S,
  /// Ecommerce ID
  pub id_ecommerce: u64,
  /// Type
  pub tipo: S,
  /// Version
  pub versao: S,
  /// Data
  pub dados: D,
}
