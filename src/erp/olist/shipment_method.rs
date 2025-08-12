/// Shipment method
#[derive(Clone, Copy, Debug, serde::Deserialize, serde::Serialize)]
pub enum ShipmentMethod {
  /// AliExpressEnvios
  #[serde(rename = "ali_envios")]
  AliExpressEnvios,
  /// AmazonDba
  #[serde(rename = "amazon_dba")]
  AmazonDba,
  /// B2WEntrega
  #[serde(rename = "b")]
  B2WEntrega,
  /// Correios
  #[serde(rename = "c")]
  Correios,
  /// CorreiosEFulfillment
  #[serde(rename = "e")]
  CorreiosEFulfillment,
  /// ConectaLaEtiquetas
  #[serde(rename = "d")]
  ConectaLaEtiquetas,
  /// GatewayLogistico
  #[serde(rename = "gateway")]
  GatewayLogistico,
  /// Jadlog
  #[serde(rename = "j")]
  Jadlog,
  /// Loggi
  #[serde(rename = "loggi")]
  Loggi,
  /// MagaluEntregas
  #[serde(rename = "magalu_entregas")]
  MagaluEntregas,
  /// MagaluEntregasPorNetshoes
  #[serde(rename = "ns_magalu_entregas")]
  MagaluEntregasPorNetshoes,
  /// MagaluFulfillment
  #[serde(rename = "magalu_fulfillment")]
  MagaluFulfillment,
  /// MadeiraEnvios
  #[serde(rename = "madeira_envios")]
  MadeiraEnvios,
  /// MercadoEnvios
  #[serde(rename = "m")]
  MercadoEnvios,
  /// NetshoesEntregas
  #[serde(rename = "ns_entregas")]
  NetshoesEntregas,
  /// Olist
  #[serde(rename = "olist")]
  Olist,
  /// SemFrete
  #[serde(rename = "s")]
  SemFrete,
  /// ShopeeEnvios
  #[serde(rename = "shopee_envios")]
  ShopeeEnvios,
  /// Transportadora
  #[serde(rename = "t")]
  Transportadora,
  /// TotalExpress
  #[serde(rename = "totalexpress")]
  TotalExpress,
  /// ViaVarejoEnvvias
  #[serde(rename = "viavarejo_envvias")]
  ViaVarejoEnvvias,
  /// Customizada
  #[serde(rename = "x")]
  Customizada,
}
