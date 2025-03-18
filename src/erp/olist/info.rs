use crate::erp::olist::TributaryRegime;

/// Represents a company or business entity with its relevant information
#[derive(Debug, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Info<S> {
  /// Company legal name
  pub razao_social: S,
  /// CPF (individual taxpayer ID) or CNPJ (company taxpayer ID)
  pub cpf_cnpj: S,
  /// Trade name or fantasy name (nome fantasia)
  pub fantasia: S,
  /// Company's address information
  pub endereco_empresa: InfoAddress<S>,
  /// Phone number
  pub fone: S,
  /// Company's email address
  pub email: S,
  /// State registration number (inscrição estadual)
  pub inscricao_estadual: S,
  /// Tax regime code
  pub regime_tributario: TributaryRegime,
}

/// Represents a complete address structure
#[derive(Debug, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct InfoAddress<S> {
  /// Street name or address description
  /// Example: "Rua das Flores"
  pub endereco: S,
  /// Street number
  pub numero: S,
  /// Address complement (optional)
  pub complemento: S,
  /// Neighborhood or district
  pub bairro: S,
  /// City or municipality
  pub municipio: S,
  /// Postal code (CEP)
  pub cep: S,
  /// State abbreviation (UF)
  pub uf: S,
  /// Country name
  pub pais: S,
}
