use rust_decimal::Decimal;
use wtx::collection::Vector;

/// Free shipping method.
#[derive(Debug, serde::Deserialize, serde::Serialize)]
pub struct FreeMethod {
  /// Free shipping method identifier.
  pub id: Option<u64>,
}

/// Information about the delivery address.
#[derive(Debug, serde::Deserialize, serde::Serialize)]
pub struct ReceiverAddress<T> {
  /// Delivery address city name.
  pub city_name: Option<T>,
  /// Delivery address country name.
  pub country_name: Option<T>,
  /// Delivery address state name.
  pub state_name: Option<T>,
  /// Delivery address street name.
  pub street_name: Option<T>,
  /// Delivery address street number.
  pub street_number: Option<T>,
  /// Delivery address zip code.
  pub zip_code: Option<T>,
}

/// Shipping information.
#[derive(Debug, serde::Deserialize, serde::Serialize)]
pub struct Shipments<T> {
  /// Shipping cost.
  #[serde(default, with = "rust_decimal::serde::float_option")]
  pub cost: Option<Decimal>,
  /// Default shipping method.
  pub default_shipping_method: Option<u64>,
  /// Package dimensions.
  pub dimensions: Option<T>,
  /// Free shipping methods.
  pub free_methods: Option<Vector<FreeMethod>>,
  /// Indicates if the shipping is free.
  pub free_shipping: Option<bool>,
  /// Indicates if the shipping is local.
  pub local_pickup: Option<bool>,
  /// Information about the delivery address.
  pub receiver_address: Option<ReceiverAddress<T>>,
}
