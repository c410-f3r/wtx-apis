/// Payment method
#[derive(Debug, Clone, Copy, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
pub enum PaymentMethod {
  /// BankSlip
  #[serde(rename = "boleto")]
  BankSlip,
  /// Cash
  #[serde(rename = "dinheiro")]
  Cash,
  /// Check
  #[serde(rename = "cheque")]
  Check,
  /// CommercialInvoice
  #[serde(rename = "duplicata_mercantil")]
  CommercialInvoice,
  /// Credit
  #[serde(rename = "credito")]
  Credit,
  /// Debit
  #[serde(rename = "debito")]
  Debit,
  /// Deposit
  #[serde(rename = "deposito")]
  Deposit,
  /// InstallmentPlan
  #[serde(rename = "crediario")]
  InstallmentPlan,
  /// Multiple payments
  #[serde(rename = "multiplas")]
  Multiple,
  /// MPix
  #[serde(rename = "pix")]
  Pix,
}
