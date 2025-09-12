use rust_decimal::Decimal;

/// Inventory webhook
#[derive(Debug, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct WebhookProductInventory<S> {
  /// Stock type
  pub tipo_estoque: S,
  /// Balance
  pub saldo: Decimal,
  /// ID product
  pub id_produto: u64,
  /// SKU
  pub sku: S,
  /// Mapping SKU
  pub sku_mapeamento: S,
  /// Mapping SKU of parent
  pub sku_mapeamento_pai: Option<S>,
}

/// Product mapping webhook
#[derive(Debug, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct WebhookProductMapping<S> {
  /// Error
  pub error: Option<S>,
  /// Mapping id
  pub id_mapeamento: u64,
  /// Mapping SKU
  pub sku_mapeamento: S,
  /// Image URL
  pub url_imagem: Option<S>,
  /// Product URL
  pub url_produto: Option<S>,
}

/// Price webhook
#[derive(Debug, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct WebhookProductPrice<S> {
  /// Mapping id
  pub id_mapeamento: u64,
  /// Mapping SKU
  pub sku_mapeamento: S,
  /// Mapping SKU of parent
  pub sku_mapeamento_pai: Option<S>,
  /// Name
  pub nome: S,
  /// Code
  pub codigo: Option<S>,
  /// Price
  pub preco: Decimal,
  /// Promotional price
  pub preco_promocional: Option<Decimal>,
}

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

/// Price webhook
#[derive(Debug, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct WebhookOrderInvoice<S> {
  /// Order ID
  pub chave_acesso: Option<S>,
  /// Mapping SKU
  pub numero: Option<S>,
  /// Mapping SKU
  pub serie: Option<S>,
  /// Mapping SKU of parent
  pub url_danfe: Option<S>,
  /// Name
  pub id_pedido_ecommerce: Option<S>,
  /// Name
  pub data_emissao: Option<S>,
  /// Name
  pub valor_nota: Option<S>,
  /// Name
  pub id_nota_fiscal_tiny: Option<S>,
}

/// Price webhook
#[derive(Debug, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct WebhookOrderStatus<S> {
  /// Order ID
  pub id_pedido_ecommerce: Option<S>,
  /// Selling ID
  pub id_venda_tiny: u64,
  /// Situation
  pub situacao: Option<S>,
  /// Description
  pub descricao_situacao: Option<S>,
}

#[cfg(test)]
mod tests {
  use crate::erp::olist::{
    ProductWebhook, WebhookOrderInvoice, WebhookOrderStatus, WebhookRequest,
    webhook_request::{WebhookProductInventory, WebhookProductPrice},
  };

  #[test]
  fn json() {
    let _elem: WebhookRequest<WebhookOrderInvoice<&str>, &str> =
      serde_json::from_str(include_str!("../../../assets/olist/webhook-order-invoice.json"))
        .unwrap();
    let _elem: WebhookRequest<WebhookOrderStatus<&str>, &str> =
      serde_json::from_str(include_str!("../../../assets/olist/webhook-order-status.json"))
        .unwrap();
    let _elem: WebhookRequest<ProductWebhook<&str>, &str> =
      serde_json::from_str(include_str!("../../../assets/olist/webhook-product.json")).unwrap();
    let _elem: WebhookRequest<WebhookProductInventory<&str>, &str> =
      serde_json::from_str(include_str!("../../../assets/olist/webhook-product-inventory.json"))
        .unwrap();
    let _elem: WebhookRequest<WebhookProductPrice<&str>, &str> =
      serde_json::from_str(include_str!("../../../assets/olist/webhook-product-price.json"))
        .unwrap();
  }
}
