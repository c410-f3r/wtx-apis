use crate::erp::olist::{AssociatedEntityId, CivilStatus, Gender, PersonTy, TributaryRegime};
use rust_decimal::Decimal;
use wtx::misc::Vector;

/// Represents the current situation/status of a contact
#[derive(Clone, Copy, Debug, PartialEq, serde::Deserialize, serde::Serialize)]
pub enum ContactSituation {
  /// Unknown
  #[serde(rename = "A")]
  ActiveWithSystemAccess,
  /// Unknown
  #[serde(rename = "B")]
  Active,
  /// Inactive
  #[serde(rename = "I")]
  Inactive,
  /// No actions
  #[serde(rename = "E")]
  Excluded,
}

/// Represents a contact entity with detailed personal and business information
#[derive(Debug, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ContactPost<S> {
  /// Full name of the contact
  pub nome: S,
  /// Unique code or identifier for the contact
  #[serde(skip_serializing_if = "Option::is_none")]
  pub codigo: Option<S>,
  /// Trade name or business name
  #[serde(skip_serializing_if = "Option::is_none")]
  pub fantasia: Option<S>,
  /// Type of person
  pub tipo_pessoa: PersonTy,
  /// CPF (individual) or CNPJ (business) number
  #[serde(skip_serializing_if = "Option::is_none")]
  pub cpf_cnpj: Option<S>,
  /// State registration number
  #[serde(skip_serializing_if = "Option::is_none")]
  pub inscricao_estadual: Option<S>,
  /// RG (identity document) number
  #[serde(skip_serializing_if = "Option::is_none")]
  pub rg: Option<S>,
  /// Primary phone number
  #[serde(skip_serializing_if = "Option::is_none")]
  pub telefone: Option<S>,
  /// Mobile phone number
  #[serde(skip_serializing_if = "Option::is_none")]
  pub celular: Option<S>,
  /// Primary email address
  #[serde(skip_serializing_if = "Option::is_none")]
  pub email: Option<S>,
  /// Main address information
  #[serde(skip_serializing_if = "Option::is_none")]
  pub endereco: Option<ContactPostAddress<S>>,
  /// Billing address information (optional)
  #[serde(skip_serializing_if = "Option::is_none")]
  pub endereco_cobranca: Option<ContactPostAddress<S>>,
  /// Municipal registration number
  #[serde(skip_serializing_if = "Option::is_none")]
  pub inscricao_municipal: Option<S>,
  /// Additional phone number
  #[serde(skip_serializing_if = "Option::is_none")]
  pub telefone_adicional: Option<S>,
  /// Email address for electronic invoices (NFe)
  #[serde(skip_serializing_if = "Option::is_none")]
  pub email_nfe: Option<S>,
  /// Website URL
  #[serde(skip_serializing_if = "Option::is_none")]
  pub site: Option<S>,
  /// Tax regime
  #[serde(skip_serializing_if = "Option::is_none")]
  pub regime_tributario: Option<TributaryRegime>,
  /// Marital status
  #[serde(skip_serializing_if = "Option::is_none")]
  pub estado_civil: Option<CivilStatus>,
  /// Occupation or profession
  #[serde(skip_serializing_if = "Option::is_none")]
  pub profissao: Option<S>,
  /// Gender
  #[serde(skip_serializing_if = "Option::is_none")]
  pub sexo: Option<Gender>,
  /// Date of birth
  #[serde(skip_serializing_if = "Option::is_none")]
  pub data_nascimento: Option<S>,
  /// Place of birth
  #[serde(skip_serializing_if = "Option::is_none")]
  pub naturalidade: Option<S>,
  /// Father's name
  #[serde(skip_serializing_if = "Option::is_none")]
  pub nome_pai: Option<S>,
  /// Mother's name
  #[serde(skip_serializing_if = "Option::is_none")]
  pub nome_mae: Option<S>,
  /// Father's CPF
  #[serde(skip_serializing_if = "Option::is_none")]
  pub cpf_pai: Option<S>,
  /// Mother's CPF
  #[serde(skip_serializing_if = "Option::is_none")]
  pub cpf_mae: Option<S>,
  /// Credit limit
  #[serde(skip_serializing_if = "Option::is_none")]
  pub limite_credito: Option<Decimal>,
  /// Contact status
  #[serde(skip_serializing_if = "Option::is_none")]
  pub situacao: Option<ContactSituation>,
  /// Additional observations or notes
  #[serde(skip_serializing_if = "Option::is_none")]
  pub observacoes: Option<S>,
  /// Associated vendor information
  #[serde(skip_serializing_if = "Option::is_none")]
  pub vendedor: Option<AssociatedEntityId>,
  /// List of contact types
  #[serde(skip_serializing_if = "Option::is_none")]
  pub tipos: Option<Vector<u32>>,
  /// List of additional contacts
  #[serde(skip_serializing_if = "Option::is_none")]
  pub contatos: Option<Vector<ContactPostContact<S>>>,
}

/// Represents address information
#[derive(Debug, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct ContactPostAddress<S> {
  /// Street address
  #[serde(skip_serializing_if = "Option::is_none")]
  pub endereco: Option<S>,
  /// Street number
  #[serde(skip_serializing_if = "Option::is_none")]
  pub numero: Option<S>,
  /// Address complement
  #[serde(skip_serializing_if = "Option::is_none")]
  pub complemento: Option<S>,
  /// Neighborhood
  #[serde(skip_serializing_if = "Option::is_none")]
  pub bairro: Option<S>,
  /// City
  #[serde(skip_serializing_if = "Option::is_none")]
  pub municipio: Option<S>,
  /// Postal code (CEP)
  #[serde(skip_serializing_if = "Option::is_none")]
  pub cep: Option<S>,
  /// State (UF)
  #[serde(skip_serializing_if = "Option::is_none")]
  pub uf: Option<S>,
  /// Country
  #[serde(skip_serializing_if = "Option::is_none")]
  pub pais: Option<S>,
}

/// Represents additional contact information
#[derive(Debug, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct ContactPostContact<S> {
  /// Contact name
  #[serde(skip_serializing_if = "Option::is_none")]
  pub nome: Option<S>,
  /// Contact phone number
  #[serde(skip_serializing_if = "Option::is_none")]
  pub telefone: Option<S>,
  /// Phone extension
  #[serde(skip_serializing_if = "Option::is_none")]
  pub ramal: Option<S>,
  /// Contact email
  #[serde(skip_serializing_if = "Option::is_none")]
  pub email: Option<S>,
  /// Department or sector
  #[serde(skip_serializing_if = "Option::is_none")]
  pub setor: Option<S>,
}
