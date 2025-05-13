use crate::erp::olist::{AssociatedEntityId, FreightResponsibility, OrderStatus, PersonTy};
use rust_decimal::Decimal;
use wtx::{collection::Vector, time::DateTime};

/// Derive common traits for serialization, deserialization, and debugging
#[derive(Debug, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct OrderPost<S> {
  /// Expected date
  #[serde(skip_serializing_if = "Option::is_none", with = "crate::misc::yyyy_mm_dd_opt")]
  pub data_prevista: Option<DateTime>,
  /// Shipment date
  #[serde(skip_serializing_if = "Option::is_none", with = "crate::misc::yyyy_mm_dd_opt")]
  pub data_envio: Option<DateTime>,
  /// General observations/comments
  #[serde(skip_serializing_if = "Option::is_none")]
  pub observacoes: Option<S>,
  /// Internal observations/comments
  #[serde(skip_serializing_if = "Option::is_none")]
  pub observacoes_internas: Option<S>,
  /// Order status
  #[serde(skip_serializing_if = "Option::is_none")]
  pub situacao: Option<OrderStatus>,
  /// Order date
  #[serde(skip_serializing_if = "Option::is_none", with = "crate::misc::yyyy_mm_dd_opt")]
  pub data: Option<DateTime>,
  /// Delivery date
  #[serde(skip_serializing_if = "Option::is_none", with = "crate::misc::yyyy_mm_dd_opt")]
  pub data_entrega: Option<DateTime>,
  /// Purchase order number
  #[serde(skip_serializing_if = "Option::is_none")]
  pub numero_ordem_compra: Option<S>,
  /// Discount amount
  #[serde(skip_serializing_if = "Option::is_none")]
  pub valor_desconto: Option<Decimal>,
  /// Freight/shipping cost
  #[serde(skip_serializing_if = "Option::is_none")]
  pub valor_frete: Option<Decimal>,
  /// Other expenses
  #[serde(skip_serializing_if = "Option::is_none")]
  pub valor_outras_despesas: Option<Decimal>,
  /// Contact ID
  pub id_contato: u32,
  /// Price list details
  #[serde(skip_serializing_if = "Option::is_none")]
  pub lista_preco: Option<AssociatedEntityId>,
  /// Operation type details
  #[serde(skip_serializing_if = "Option::is_none")]
  pub natureza_operacao: Option<AssociatedEntityId>,
  /// Seller details
  #[serde(skip_serializing_if = "Option::is_none")]
  pub vendedor: Option<AssociatedEntityId>,
  /// Delivery address details
  #[serde(skip_serializing_if = "Option::is_none")]
  pub endereco_entrega: Option<OrderPostAddress<S>>,
  /// E-commerce details
  #[serde(skip_serializing_if = "Option::is_none")]
  pub ecommerce: Option<OrderPostEcommerce<S>>,
  /// Carrier/transporter details
  #[serde(skip_serializing_if = "Option::is_none")]
  pub transportador: Option<OrderPostCarrier<S>>,
  /// Intermediary details
  #[serde(skip_serializing_if = "Option::is_none")]
  pub intermediador: Option<AssociatedEntityId>,
  /// Warehouse/deposit details
  #[serde(skip_serializing_if = "Option::is_none")]
  pub deposito: Option<AssociatedEntityId>,
  /// Payment details
  #[serde(skip_serializing_if = "Option::is_none")]
  pub pagamento: Option<OrderPostPayment<S>>,
  /// List of order items
  #[serde(skip_serializing_if = "Option::is_none")]
  pub itens: Option<Vector<OrderPostItem<S>>>,
}

/// Delivery address struct
#[derive(Debug, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct OrderPostAddress<S> {
  /// Street address
  #[serde(skip_serializing_if = "Option::is_none")]
  pub endereco: Option<S>,
  /// Street number
  #[serde(skip_serializing_if = "Option::is_none")]
  pub endereco_nro: Option<S>,
  /// Address complement
  #[serde(skip_serializing_if = "Option::is_none")]
  pub complemento: Option<S>,
  /// Neighborhood
  #[serde(skip_serializing_if = "Option::is_none")]
  pub bairro: Option<S>,
  /// City/municipality
  #[serde(skip_serializing_if = "Option::is_none")]
  pub municipio: Option<S>,
  /// ZIP code
  #[serde(skip_serializing_if = "Option::is_none")]
  pub cep: Option<S>,
  /// State
  #[serde(skip_serializing_if = "Option::is_none")]
  pub uf: Option<S>,
  /// Phone number
  #[serde(skip_serializing_if = "Option::is_none")]
  pub fone: Option<S>,
  /// Recipient name
  #[serde(skip_serializing_if = "Option::is_none")]
  pub nome_destinatario: Option<S>,
  /// CPF or CNPJ (tax ID)
  #[serde(skip_serializing_if = "Option::is_none")]
  pub cpf_cnpj: Option<S>,
  /// Person type
  #[serde(skip_serializing_if = "Option::is_none")]
  pub tipo_pessoa: Option<PersonTy>,
  /// State registration (Inscrição Estadual)
  #[serde(skip_serializing_if = "Option::is_none")]
  pub ie: Option<S>,
}

/// Carrier/transporter struct
#[derive(Debug, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct OrderPostCarrier<S> {
  /// Carrier ID
  #[serde(skip_serializing_if = "Option::is_none")]
  pub id: Option<u32>,
  /// Freight responsibility
  #[serde(skip_serializing_if = "Option::is_none")]
  pub frete_por_conta: Option<FreightResponsibility>,
  /// Shipping method details
  #[serde(skip_serializing_if = "Option::is_none")]
  pub forma_envio: Option<AssociatedEntityId>,
  /// Freight type details
  #[serde(skip_serializing_if = "Option::is_none")]
  pub forma_frete: Option<AssociatedEntityId>,
  /// Tracking code
  #[serde(skip_serializing_if = "Option::is_none")]
  pub codigo_rastreamento: Option<S>,
  /// Tracking URL
  #[serde(skip_serializing_if = "Option::is_none")]
  pub url_rastreamento: Option<S>,
}

/// E-commerce struct
#[derive(Debug, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct OrderPostEcommerce<S> {
  /// E-commerce ID
  #[serde(skip_serializing_if = "Option::is_none")]
  pub id: Option<u32>,
  /// E-commerce order number
  #[serde(skip_serializing_if = "Option::is_none")]
  pub numero_pedido_ecommerce: Option<S>,
}

/// Payment installment struct
#[derive(Debug, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct OrderPostInstallment<S> {
  /// Number of days until payment
  #[serde(skip_serializing_if = "Option::is_none")]
  pub dias: Option<u32>,
  /// Payment date
  #[serde(skip_serializing_if = "Option::is_none", with = "crate::misc::yyyy_mm_dd_opt")]
  pub data: Option<DateTime>,
  /// Installment amount (decimal)
  #[serde(skip_serializing_if = "Option::is_none")]
  pub valor: Option<Decimal>,
  /// Installment observations/comments
  #[serde(skip_serializing_if = "Option::is_none")]
  pub observacoes: Option<S>,
  /// Payment method details for this installment
  #[serde(skip_serializing_if = "Option::is_none")]
  pub forma_pagamento: Option<AssociatedEntityId>,
  /// Payment means details for this installment
  #[serde(skip_serializing_if = "Option::is_none")]
  pub meio_pagamento: Option<AssociatedEntityId>,
}

/// Order item struct
#[derive(Debug, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct OrderPostItem<S> {
  /// Product details
  #[serde(skip_serializing_if = "Option::is_none")]
  pub produto: Option<AssociatedEntityId>,
  /// Quantity
  #[serde(skip_serializing_if = "Option::is_none")]
  pub quantidade: Option<Decimal>,
  /// Unit price (decimal)
  #[serde(skip_serializing_if = "Option::is_none")]
  pub valor_unitario: Option<Decimal>,
  /// Additional information about the item
  #[serde(skip_serializing_if = "Option::is_none")]
  pub info_adicional: Option<S>,
}

/// Payment struct
#[derive(Debug, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct OrderPostPayment<S> {
  /// Payment method details
  #[serde(skip_serializing_if = "Option::is_none")]
  pub forma_pagamento: Option<AssociatedEntityId>,
  /// Payment means details
  #[serde(skip_serializing_if = "Option::is_none")]
  pub meio_pagamento: Option<AssociatedEntityId>,
  /// List of payment installments
  #[serde(skip_serializing_if = "Option::is_none")]
  pub parcelas: Option<Vector<OrderPostInstallment<S>>>,
}
