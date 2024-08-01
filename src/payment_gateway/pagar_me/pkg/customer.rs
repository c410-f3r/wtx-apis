#[wtx_macros::pkg(
  api(crate::payment_gateway::pagar_me::PagarMe),
  data_format(json),
  transport(http)
)]
pub(crate) mod pkg {
  use crate::payment_gateway::pagar_me::{
    address::{AddressOwned, AddressRef},
    phone::{PhoneOwned, PhoneRef},
    PagarMeHttpPkgsAux,
  };
  use wtx::misc::{ArrayString, ArrayVector};

  #[pkg::aux]
  impl<DRSR> PagarMeHttpPkgsAux<DRSR> {}

  /// Customer request data
  #[derive(Debug, serde::Serialize)]
  #[pkg::req_data]
  #[serde(rename_all = "camelCase")]
  pub struct CustomerReq<'any> {
    /// See [Address].
    pub address: AddressRef<'any>,
    /// DDMMYYYY
    pub born_at: &'any str,
    /// Document number
    pub document_number: &'any str,
    /// Email
    pub email: &'any str,
    /// F/M
    pub gender: char,
    /// Name
    pub name: &'any str,
    /// See [Phone]
    pub phone: PhoneRef<'any>,
  }

  #[derive(Debug)]
  #[pkg::res_data]
  pub struct CustomerRes {
    /// See [AddressOwned]
    pub addresses: ArrayVector<AddressOwned, 4>,
    /// DDMMYYYY
    pub born_at: ArrayString<8>,
    /// DDMMYYYY
    pub date_created: ArrayString<8>,
    /// Document number
    pub document_number: ArrayString<16>,
    /// See [DocumentType]
    pub document_type: DocumentType,
    /// Email
    pub email: ArrayString<32>,
    /// F/M
    pub gender: char,
    /// Identifier
    pub id: u64,
    /// Name
    pub name: ArrayString<64>,
    /// See [PhoneOwned]
    pub phones: ArrayVector<PhoneOwned, 4>,
  }

  #[derive(Debug, serde::Deserialize)]
  #[serde(rename_all = "lowercase")]
  pub enum DocumentType {
    /// CNPJ
    Cnpj,
    /// CPF
    Cpf,
  }

  #[derive(Debug, serde::Deserialize)]
  #[serde(rename_all = "snake_case")]
  pub enum BankAccountType {
    /// Conta Corrente
    ContaCorrente,
    /// Conta Corrent Conjunta
    ContaCorrenteConjunta,
    /// Conta Poupança
    ContaPoupanca,
    /// Conta Poupança Conjunta
    ContaPoupancaConjunta,
  }
}
