use crate::erp::olist::{
  FreightResponsibility, OrderStatus, PersonTy, associated_entity::AssociatedEntityIdName,
};
use chrono::NaiveDate;
use rust_decimal::Decimal;
use wtx::misc::Vector;

/// Represents an order with various optional fields.
#[derive(Debug, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OrderGet<S> {
  /// The expected date.
  #[serde(with = "crate::misc::yyyy_mm_dd_opt")]
  pub data_prevista: Option<NaiveDate>,
  /// The sending date.
  #[serde(with = "crate::misc::yyyy_mm_dd_opt")]
  pub data_envio: Option<NaiveDate>,
  /// Observations.
  pub observacoes: Option<S>,
  /// Internal observations.
  pub observacoes_internas: Option<S>,
  /// Situation.
  pub situacao: Option<OrderStatus>,
  /// Date.
  #[serde(with = "crate::misc::yyyy_mm_dd_opt")]
  pub data: Option<NaiveDate>,
  /// Delivery date.
  #[serde(with = "crate::misc::yyyy_mm_dd_opt")]
  pub data_entrega: Option<NaiveDate>,
  /// Purchase order number.
  pub numero_ordem_compra: Option<S>,
  /// Discount value.
  pub valor_desconto: Option<Decimal>,
  /// Freight value.
  pub valor_frete: Option<Decimal>,
  /// Other expenses value.
  pub valor_outras_despesas: Option<Decimal>,
  /// ID.
  pub id: Option<u32>,
  /// Order number.
  pub numero_pedido: Option<u32>,
  /// Invoice ID.
  pub id_nota_fiscal: Option<u32>,
  /// Billing date.
  #[serde(with = "crate::misc::yyyy_mm_dd_opt")]
  pub data_faturamento: Option<NaiveDate>,
  /// Total product value.
  pub valor_total_produtos: Option<Decimal>,
  /// Total order value.
  pub valor_total_pedido: Option<Decimal>,
  /// Price list.
  pub lista_preco: Option<OrderGetPriceList<S>>,
  /// Client information.
  pub cliente: Option<OrderGetClient<S>>,
  /// Delivery address.
  pub endereco_entrega: Option<OrderGetDeliveryAddress<S>>,
  /// E-commerce information.
  pub ecommerce: Option<OrderGetEcommerce<S>>,
  /// Transporter information.
  pub transportador: Option<OrderGetTransporter<S>>,
  /// Depository information.
  pub deposito: Option<AssociatedEntityIdName<S>>,
  /// Seller information.
  pub vendedor: Option<AssociatedEntityIdName<S>>,
  /// Nature of the operation.
  pub natureza_operacao: Option<AssociatedEntityIdName<S>>,
  /// Intermediary information.
  pub intermediador: Option<OrderGetIntermediary<S>>,
  /// Payment information.
  pub pagamento: Option<OrderGetPayment<S>>,
  /// List of items.
  pub itens: Option<Vector<OrderGetItem<S>>>,
}

/// Represents a client.
#[derive(Debug, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OrderGetClient<S> {
  /// Name
  pub nome: Option<S>,
  /// Code
  pub codigo: Option<S>,
  /// Business name
  pub fantasia: Option<S>,
  /// See [PersonTy].
  pub tipo_pessoa: Option<PersonTy>,
  /// Federal registration
  pub cpf_cnpj: Option<S>,
  /// State registration
  pub inscricao_estadual: Option<S>,
  /// Person identifier
  pub rg: Option<S>,
  /// Phone
  pub telefone: Option<S>,
  /// Cellphone
  pub celular: Option<S>,
  /// Email
  pub email: Option<S>,
  /// Street
  pub endereco: Option<OrderGetClientAddress<S>>,
  /// ID
  pub id: Option<u32>,
}

/// Represents an address.
#[derive(Debug, serde::Deserialize)]
pub struct OrderGetClientAddress<S> {
  /// Street
  pub endereco: Option<S>,
  /// Street number
  pub numero: Option<S>,
  /// Complement
  pub complemento: Option<S>,
  /// Neighbor
  pub bairro: Option<S>,
  /// District
  pub municipio: Option<S>,
  /// Postal code
  pub cep: Option<S>,
  /// State
  pub uf: Option<S>,
  /// Country
  pub pais: Option<S>,
}

/// Represents a delivery address.
#[derive(Debug, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OrderGetDeliveryAddress<S> {
  /// Street
  pub endereco: Option<S>,
  /// Number
  pub numero: Option<S>,
  /// Complement
  pub complemento: Option<S>,
  /// Neighbor
  pub bairro: Option<S>,
  /// District
  pub municipio: Option<S>,
  /// Postal code
  pub cep: Option<S>,
  /// State
  pub uf: Option<S>,
  /// Country
  pub pais: Option<S>,
  /// Destiny person name
  pub nome_destinatario: Option<S>,
  /// Registration
  pub cpf_cnpj: Option<S>,
  /// See [PersonTy]
  pub tipo_pessoa: Option<PersonTy>,
}

/// Represents e-commerce information.
#[derive(Debug, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OrderGetEcommerce<S> {
  /// Id
  pub id: Option<u32>,
  /// Name
  pub nome: Option<S>,
  /// Ecommerce order number
  pub numero_pedido_ecommerce: Option<S>,
  /// Vendor order number
  pub numero_pedido_canal_venda: Option<S>,
  /// Vendor
  pub canal_venda: Option<S>,
}

/// Represents an intermediary.
#[derive(Debug, serde::Deserialize)]
pub struct OrderGetIntermediary<S> {
  /// ID
  pub id: Option<u32>,
  /// Name
  pub nome: Option<S>,
  /// Registration
  pub cnpj: Option<S>,
}

/// Represents an item.
#[derive(Debug, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OrderGetItem<S> {
  /// Product
  pub produto: Option<OrderGetItemProduct<S>>,
  /// Quantity
  pub quantidade: Option<Decimal>,
  /// Unit value
  pub valor_unitario: Option<Decimal>,
  /// Additional information
  pub info_adicional: Option<S>,
}

/// Represents a product.
#[derive(Debug, serde::Deserialize)]
pub struct OrderGetItemProduct<S> {
  /// ID
  pub id: Option<u32>,
  /// Stock keeping unit
  pub sku: Option<S>,
  /// Description
  pub descricao: Option<S>,
}

/// Represents payment information.
#[derive(Debug, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OrderGetPayment<S> {
  /// Payment type
  pub forma_pagamento: Option<AssociatedEntityIdName<S>>,
  /// Payment form
  pub meio_pagamento: Option<AssociatedEntityIdName<S>>,
  /// Payment condition
  pub condicao_pagamento: Option<S>,
  /// Installments
  pub parcelas: Option<Vector<OrderGetPaymentInstallment<S>>>,
}

/// Represents an installment.
#[derive(Debug, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OrderGetPaymentInstallment<S> {
  /// Days
  pub dias: Option<u16>,
  /// Date
  #[serde(with = "crate::misc::yyyy_mm_dd_opt")]
  pub data: Option<NaiveDate>,
  /// Value
  pub valor: Option<Decimal>,
  /// Observations
  pub observacoes: Option<S>,
  /// Payment type
  pub forma_pagamento: Option<AssociatedEntityIdName<S>>,
  /// Payment form
  pub meio_pagamento: Option<AssociatedEntityIdName<S>>,
}

/// Represents a price list.
#[derive(Debug, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OrderGetPriceList<S> {
  /// ID
  pub id: Option<u32>,
  /// Name
  pub nome: Option<S>,
  /// Additional discount
  pub acrescimo_desconto: Option<Decimal>,
}

/// Represents a transporter.
#[derive(Debug, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OrderGetTransporter<S> {
  /// ID
  pub id: Option<u32>,
  /// Name
  pub nome: Option<S>,
  /// See [FreightResponsibility]
  pub frete_por_conta: Option<FreightResponsibility>,
  /// Sending type
  pub forma_envio: Option<AssociatedEntityIdName<S>>,
  /// Freight type
  pub forma_frete: Option<AssociatedEntityIdName<S>>,
  /// Tracking code
  pub codigo_rastreamento: Option<S>,
  /// Tracking URL
  pub url_rastreamento: Option<S>,
}
