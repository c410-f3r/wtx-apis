use rust_decimal::Decimal;
use wtx::misc::Vector;

/// Represents package type
#[derive(Debug, serde::Deserialize, serde::Serialize)]
pub enum PackageType {
  /// Envelope
  #[serde(rename = "1")]
  Envelope,
  /// Box
  #[serde(rename = "2")]
  Box,
  /// Cylinder
  #[serde(rename = "3")]
  Cylinder,
}

/// Represents product class
#[derive(Debug, serde::Deserialize, serde::Serialize)]
pub enum ProductClass {
  /// Simple
  #[serde(rename = "S")]
  Simple,
  /// Kit
  #[serde(rename = "K")]
  Kit,
  /// With variants
  #[serde(rename = "V")]
  WithVariations,
  /// Manufactured
  #[serde(rename = "F")]
  Manufactured,
  /// Raw material
  #[serde(rename = "M")]
  RawMaterial,
}

/// Represents product status
#[derive(Debug, serde::Deserialize, serde::Serialize)]
pub enum ProductStatus {
  /// Active
  #[serde(rename = "A")]
  Active,
  /// Inactive
  #[serde(rename = "I")]
  Inactive,
}

/// Represents product type
#[derive(Debug, serde::Deserialize, serde::Serialize)]
pub enum ProductType {
  /// Product
  #[serde(rename = "P")]
  Product,
  /// Service
  #[serde(rename = "S")]
  Service,
}

/// Represents a product with all its details including variations, attachments, and categories
#[derive(Debug, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ProductWebhook<S> {
  /// Unique identifier for the product
  pub id: Option<S>,
  /// Mapping identifier
  pub id_mapeamento: Option<S>,
  /// SKU mapping identifier (can be empty)
  pub sku_mapeamento: Option<S>,
  /// Product name
  pub nome: Option<S>,
  /// Product code
  pub codigo: Option<S>,
  /// Unit of measurement
  pub unidade: Option<S>,
  /// Regular price with 4 decimal places
  pub preco: Option<Decimal>,
  /// Promotional price with 4 decimal places
  pub preco_promocional: Option<Decimal>,
  /// NCM (Nomenclatura Comum do Mercosul) code for tax purposes
  pub ncm: Option<S>,
  /// Product origin code
  pub origem: Option<S>,
  /// Global Trade Item Number
  pub gtin: Option<S>,
  /// GTIN for the packaging (can be empty)
  pub gtin_embalagem: Option<S>,
  /// Physical location of the product
  pub localizacao: Option<S>,
  /// Net weight
  pub peso_liquido: Option<Decimal>,
  /// Gross weight
  pub peso_bruto: Option<Decimal>,
  /// Minimum stock level
  pub estoque_minimo: Option<Decimal>,
  /// Maximum stock level
  pub estoque_maximo: Option<Decimal>,
  /// Supplier identifier
  pub id_fornecedor: Option<S>,
  /// Supplier code (can be empty)
  pub codigo_fornecedor: Option<S>,
  /// Product code used by the supplier
  pub codigo_pelo_fornecedor: Option<S>,
  /// Units per box/package
  pub unidade_por_caixa: Option<S>,
  /// Current stock quantity
  pub estoque_atual: Option<Decimal>,
  /// Cost price with 4 decimal places
  pub preco_custo: Option<Decimal>,
  /// Average cost price with 4 decimal places
  pub preco_custo_medio: Option<Decimal>,
  /// Product status
  pub situacao: Option<ProductStatus>,
  /// Complementary description with HTML formatting
  pub descricao_complementar: Option<S>,
  /// Additional observations
  pub obs: Option<S>,
  /// Warranty information
  pub garantia: Option<S>,
  /// CEST (Código Especificador da Substituição Tributária) code
  pub cest: Option<S>,
  /// Indicates if the product is made to order ("S" for yes, "N" for no)
  pub sob_encomenda: Option<S>,
  /// Brand name
  pub marca: Option<S>,
  /// Packaging type code
  pub tipo_embalagem: Option<PackageType>,
  /// Package height
  pub altura_embalagem: Option<Decimal>,
  /// Package width
  pub largura_embalagem: Option<Decimal>,
  /// Package length
  pub comprimento_embalagem: Option<Decimal>,
  /// Package diameter
  pub diametro_embalagem: Option<Decimal>,
  /// Product class
  pub classe_produto: Option<ProductClass>,
  /// Category identifier
  pub id_categoria: Option<S>,
  /// Category description
  pub descricao_categoria: Option<S>,
  /// Full category path description
  pub descricao_arvore_categoria: Option<S>,
  /// Hierarchical category structure
  pub arvore_categoria: Option<Vector<ProductWebhookCategoryNode<S>>>,
  /// Product variations (for variable products)
  pub variacoes: Option<Vector<ProductWebhookVariation<S>>>,
  /// Product attachments (images, documents, etc.)
  pub anexos: Option<Vector<ProductWebhookAttachment<S>>>,
  /// SEO (Search Engine Optimization) information
  pub seo: Option<ProductWebhookSeo<S>>,
  /// Components of a product kit
  pub kit: Option<Vector<ProductWebhookKitItem>>,
  /// Number of days required for preparation before shipping
  pub dias_preparacao: Option<u16>,
}

/// Represents an attachment (image, document, etc.)
#[derive(Debug, serde::Deserialize)]
pub struct ProductWebhookAttachment<S> {
  /// URL to the attachment
  pub url: Option<S>,
  /// Filename of the attachment
  pub nome: Option<S>,
  /// File type
  pub tipo: Option<S>,
}

/// Represents a node in the category hierarchy
#[derive(Debug, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ProductWebhookCategoryNode<S> {
  /// Category identifier
  pub id: Option<S>,
  /// Parent category identifier (0 for root categories)
  pub id_pai: Option<S>,
  /// Category name
  pub descricao: Option<S>,
  /// Full category path
  pub descricao_completa: Option<S>,
}

/// Represents an item in a product kit
#[derive(Debug, serde::Deserialize)]
pub struct ProductWebhookKitItem {
  /// Product identifier of the kit component
  pub id: Option<u32>,
  /// Quantity of this component in the kit
  pub quantidade: Option<Decimal>,
}

/// Represents SEO (Search Engine Optimization) information
#[derive(Debug, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ProductWebhookSeo<S> {
  /// SEO title
  pub title: Option<S>,
  /// Meta description
  pub description: Option<S>,
  /// Meta keywords
  pub keywords: Option<S>,
  /// URL to a related video
  pub link_video: Option<S>,
  /// URL slug for the product
  pub slug: Option<S>,
}

/// Represents a product variation
#[derive(Debug, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ProductWebhookVariation<S> {
  /// Unique identifier for the variation
  pub id: Option<S>,
  /// Mapping identifier for the variation
  pub id_mapeamento: Option<S>,
  /// SKU mapping identifier (can be empty)
  pub sku_mapeamento: Option<S>,
  /// Variation code
  pub codigo: Option<S>,
  /// Global Trade Item Number for the variation
  pub gtin: Option<S>,
  /// Regular price with 4 decimal places
  pub preco: Option<Decimal>,
  /// Promotional price with 4 decimal places
  pub preco_promocional: Option<Decimal>,
  /// Current stock quantity for this variation
  pub estoque_atual: Option<Decimal>,
  /// Attributes that define this variation
  pub grade: Option<Vector<ProductWebhookVariationAttribute<S>>>,
  /// Attachments specific to this variation
  pub anexos: Option<Vector<ProductWebhookAttachment<S>>>,
}

/// Represents an attribute of a product variation
#[derive(Debug, serde::Deserialize)]
pub struct ProductWebhookVariationAttribute<S> {
  /// Attribute name
  pub chave: Option<S>,
  /// Attribute value
  pub valor: Option<S>,
}
