use wtx::collection::ArrayVectorU8;

wtx::create_enum! {
  /// Invidial payment method
  #[derive(Debug, serde::Deserialize, serde::Serialize)]
  #[serde(rename_all = "snake_case", try_from = "&str")]
  pub enum PaymentMethodId<u8> {
    /// Amex
    Amex = (0, "amex"),
    /// Boleto Bradesco
    BolBradesco = (1, "bolbradesco"),
    /// Elo
    Elo = (2, "elo"),
    /// Hipercard
    Hipercard = (3, "hipercard"),
    /// Master
    Master = (4, "master"),
    /// Pix
    Pix = (5, "pix"),
    /// Visa
    Visa = (6, "visa"),
    /// No payment
    Empty = (99, ""),
  }
}

wtx::create_enum! {
  /// Super group of payment methods
  #[derive(Debug, serde::Deserialize, serde::Serialize)]
  #[serde(rename_all = "snake_case", try_from = "&str")]
  pub enum PaymentTypeId<u8> {
    /// Bank transfer
    BankTransfer = (0, "bank_transfer"),
    /// Credit card
    CreditCard = (1, "credit_card"),
    /// Ticket
    Ticket = (2, "ticket"),
    /// No payment
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
