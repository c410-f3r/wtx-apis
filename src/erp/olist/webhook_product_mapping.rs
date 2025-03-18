/// Basic structure for Webhook requests
#[derive(Debug, serde::Deserialize, serde::Serialize)]
pub struct WebhookProductMapping<S> {
  /// Error
  pub error: Option<S>,
  /// Mapping id
  #[serde(rename = "idMapeamento")]
  pub id_mapeamento: S,
  /// Mapping SKU
  #[serde(rename = "skuMapeamento")]
  pub sku_mapeamento: S,
  /// Image URL
  #[serde(rename = "urlImagem")]
  pub url_imagem: Option<S>,
  /// Product URL
  #[serde(rename = "urlProduto")]
  pub url_produto: Option<S>,
}
