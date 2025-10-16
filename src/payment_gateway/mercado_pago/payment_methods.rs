use wtx::collection::ArrayVectorU8;

wtx::create_enum! {
  /// Invidial payment method
  #[derive(Debug, serde::Deserialize, serde::Serialize)]
  #[serde(rename_all = "lowercase", try_from = "&str")]
  pub enum PaymentMethodId<u8> {
    /// Account money
    ///
    /// At the current time. this variant can't be used in method exclusions
    #[serde(rename = "account_money")]
    AccountMoney = (0, "account_money"),
    /// Amex
    #[serde(rename = "amex")]
    Amex = (1, "amex"),
    /// Boleto Bradesco
    #[serde(rename = "bolbradesco")]
    BolBradesco = (2, "bolbradesco"),
    /// Elo
    #[serde(rename = "elo")]
    Elo = (3, "elo"),
    /// Hipercard
    #[serde(rename = "hipercard")]
    Hipercard = (4, "hipercard"),
    /// Master
    #[serde(rename = "master")]
    Master = (5, "master"),
    /// Pix
    #[serde(rename = "pix")]
    Pix = (6, "pix"),
    /// Visa
    #[serde(rename = "visa")]
    Visa = (7, "visa"),
    /// No payment
    #[serde(rename = "")]
    Empty = (99, ""),
  }
}

wtx::create_enum! {
  /// Super group of payment methods
  #[derive(Debug, serde::Deserialize, serde::Serialize)]
  #[serde(try_from = "&str")]
  pub enum PaymentTypeId<u8> {
    /// Account Money
    ///
    /// At the current time, this variant can't be used in method exclusions
    #[serde(rename = "account_money")]
    AccountMoney = (0, "account_money"),
    /// Bank transfer
    #[serde(rename = "bank_transfer")]
    BankTransfer = (1, "bank_transfer"),
    /// Credit card
    #[serde(rename = "credit_card")]
    CreditCard = (2, "credit_card"),
    /// Debit card
    #[serde(rename = "debit_card")]
    DebitCard = (3, "debit_card"),
    /// Ticket
    #[serde(rename = "ticket")]
    Ticket = (4, "ticket"),
    /// No payment
    #[serde(rename = "")]
    Empty = (99, ""),
  }
}

/// Excluded payment method.
#[derive(Debug, serde::Deserialize, serde::Serialize)]
pub struct ExcludedPaymentMethod {
  /// Excluded payment method identifier.
  pub id: PaymentMethodId,
}

/// Excluded payment type.
#[derive(Debug, serde::Deserialize, serde::Serialize)]
pub struct ExcludedPaymentType {
  /// Excluded payment type identifier.
  pub id: PaymentTypeId,
}

/// Settings related to payment methods.
#[derive(Debug, serde::Deserialize, serde::Serialize)]
pub struct PaymentMethods {
  /// Default number of installments.
  pub default_installments: Option<u8>,
  /// Default payment method.
  pub default_payment_method_id: Option<PaymentMethodId>,
  /// Excluded payment methods.
  pub excluded_payment_methods:
    Option<ArrayVectorU8<ExcludedPaymentMethod, { PaymentMethodId::len() }>>,
  /// Excluded payment types.
  pub excluded_payment_types: Option<ArrayVectorU8<ExcludedPaymentType, { PaymentTypeId::len() }>>,
  /// Maximum number of installments.
  pub installments: Option<u8>,
}
