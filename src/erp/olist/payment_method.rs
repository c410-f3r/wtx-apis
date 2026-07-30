/// Payment method
#[derive(Debug, Clone, Copy, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
pub enum PaymentMethod {
  /// Bank slip
  #[serde(rename = "boleto")]
  BankSlip,
  /// Cash
  #[serde(rename = "dinheiro")]
  Cash,
  /// Check
  #[serde(rename = "cheque")]
  Check,
  /// Commercial invoice
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
  /// Installment plan
  #[serde(rename = "crediario")]
  InstallmentPlan,
  /// Multiple payments
  #[serde(rename = "multiplas")]
  Multiple,
  /// Pix
  #[serde(rename = "pix")]
  Pix,
}
