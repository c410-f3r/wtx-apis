use crate::payment_gateway::mercado_pago::{BackUrls, Item, Payer, PaymentMethods, Shipments};
use rust_decimal::Decimal;
use wtx::{
  calendar::{DateTime, DynTz},
  collection::Vector,
};

/// Differential pricing configuration.
#[derive(Debug, serde::Deserialize, serde::Serialize)]
pub struct DifferentialPricing {
  /// Differential pricing identifier.
  pub id: Option<u64>,
}

/// Main structure for the payment preference.
#[derive(Debug, serde::Deserialize, serde::Serialize)]
pub struct Preference<T> {
  /// Additional information.
  pub additional_info: Option<T>,
  /// Automatic redirection after approved payment.
  pub auto_return: Option<T>,
  /// URLs for redirection back to the seller's site.
  pub back_urls: Option<BackUrls<T>>,
  /// Differential pricing configuration.
  pub differential_pricing: Option<DifferentialPricing>,
  /// Indicates if the preference expires.
  pub expires: Option<bool>,
  /// Start date of the preference validity.
  pub expiration_date_from: Option<DateTime<DynTz>>,
  /// End date of the preference validity.
  pub expiration_date_to: Option<DateTime<DynTz>>,
  /// External reference for synchronization with the payment system.
  pub external_reference: Option<T>,
  /// Information about the items.
  pub items: Vector<Item<T>>,
  /// Payment origin.
  pub marketplace: Option<T>,
  /// Marketplace fee.
  #[serde(with = "rust_decimal::serde::float_option")]
  pub marketplace_fee: Option<Decimal>,
  /// Notification URL.
  pub notification_url: Option<T>,
  /// Information about the buyer.
  pub payer: Option<Payer<T>>,
  /// Settings related to payment methods.
  pub payment_methods: Option<PaymentMethods>,
  /// Shipping information.
  pub shipments: Option<Shipments<T>>,
  /// User interaction tracks.
  pub tracks: Option<Vector<Track<T>>>,
}

/// User interaction tracking in the payment flow.
#[derive(Debug, serde::Deserialize, serde::Serialize)]
pub struct Track<T> {
  /// Track type.
  #[serde(rename = "type")]
  pub track_type: Option<T>,
  /// Track values.
  pub values: Option<TrackValues<T>>,
}

/// Track values.
#[derive(Debug, serde::Deserialize, serde::Serialize)]
pub struct TrackValues<T> {
  /// Conversion identifier.
  pub conversion_id: Option<u64>,
  /// Conversion label.
  pub conversion_label: Option<T>,
  /// Pixel identifier.
  pub pixel_id: Option<T>,
}

#[cfg(test)]
mod tests {
  use crate::payment_gateway::mercado_pago::Preference;

  #[test]
  fn json() {
    let _elem: Preference<&str> = serde_json::from_str(include_str!(
      "../../../assets/mercado_pago/request/preference/preference_base.json"
    ))
    .unwrap();
  }
}
