use crate::payment_gateway::mercado_pago::{BackUrls, Item, Payer, PaymentMethods, Shipments};
use rust_decimal::Decimal;
use wtx::{
  calendar::{DateTime, DynTz},
  collection::Vector,
};

/// Struct representing a preference for MercadoPago.
#[derive(Debug, serde::Deserialize, serde::Serialize)]
pub struct PreferenceResponse<T> {
  /// Additional information.
  pub additional_info: Option<T>,
  /// Automatically return to the seller's site after approved credit card purchase.
  /// approved: Redirect only for approved credit card payments.
  /// all: Redirect for all approved credit card payments, future compatibility if default behavior changes.
  pub auto_return: Option<T>,
  /// URLs for returning to the seller's site, either automatically or via the 'Return to site' button, depending on payment status.
  /// The URL must use the "https" protocol.
  pub back_urls: Option<BackUrls<T>>,
  /// Binary mode: if TRUE, payments can only be approved or rejected. Otherwise, they can also be in process.
  pub binary_mode: Option<bool>,
  /// Exclusive ID used to identify the client. Obtained from the credentials used to create the preference. It is the Application ID.
  pub client_id: Option<T>,
  /// Exclusive ID used to identify the collector.
  pub collector_id: Option<u64>,
  /// Date of registration.
  pub date_created: Option<DateTime<DynTz>>,
  /// Date in the format "yyyy-MM-dd'T'HH:mm:ssz" indicating the start of the validity period of the preference.
  /// This can be used for limited sales, where sellers make an offer between certain dates.
  pub expiration_date_from: Option<DateTime<DynTz>>,
  /// Date in the format "yyyy-MM-dd'T'HH:mm:ssz" indicating the end of the validity period of the preference.
  /// This can be used for limited sales, where sellers make an offer between certain dates.
  pub expiration_date_to: Option<DateTime<DynTz>>,
  /// Preference determining whether a preference expires.
  pub expires: Option<bool>,
  /// Exclusive auto-generated ID that identifies the preference.
  pub id: Option<T>,
  /// URL automatically generated to open the Checkout.
  pub init_point: Option<T>,
  /// Information about the items.
  pub items: Vector<Item<T>>,
  /// Origin of the payment. This is an alphanumeric field whose default value is NONE.
  /// If the collector has their own marketplace, credentials are sent to identify it.
  pub marketplace: Option<T>,
  /// Marketplace fee charged by the application owner. It is a fixed value and its default is 0 in local currency.
  /// This property can only be provided if a valid marketplace has been defined.
  #[serde(with = "rust_decimal::serde::float_option")]
  pub marketplace_fee: Option<Decimal>,
  /// URL for notifications available to receive payment-related event notifications.
  /// Maximum allowed characters for this parameter is 248. The URL must use the "https" protocol.
  pub notification_url: Option<T>,
  /// Data type of the operation.
  /// regular_payment: Normal payment.
  /// money_transfer: Money request.
  pub operation_type: T,
  /// Information about the buyer, such as name, surname, email, phone, personal identification, address, and registration date.
  pub payer: Option<Payer<T>>,
  /// Settings related to payment methods, such as excluded payment methods, excluded payment types, default payment method, and fees.
  pub payment_methods: Option<PaymentMethods>,
  /// URL automatically generated to open the Checkout in sandbox mode.
  /// This parameter is deprecated and its usage is not recommended.
  pub sandbox_init_point: Option<T>,
  /// Shipments information.
  pub shipments: Option<Shipments<T>>,
  /// The statement descriptor is a long text (up to 16 characters) that will appear on the payer's credit card statement to easily identify the purchase.
  pub statement_descriptor: Option<T>,
}

#[cfg(test)]
mod tests {
  use crate::payment_gateway::mercado_pago::PreferenceResponse;

  #[test]
  fn json() {
    let _elem: PreferenceResponse<&str> = serde_json::from_str(include_str!(
      "../../../assets/mercado_pago/response/preference/preference_base.json"
    ))
    .unwrap();
    let _elem: PreferenceResponse<&str> = serde_json::from_str(include_str!(
      "../../../assets/mercado_pago/response/preference/preference_updated.json"
    ))
    .unwrap();
  }
}
