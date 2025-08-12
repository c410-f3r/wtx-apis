use rust_decimal::Decimal;

/// Freight response Status
#[derive(Debug, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum SendFreightResStatus {
  /// Canceled
  Canceled,
  /// Delivered
  Delivered,
  /// Waiting payment
  Pending,
  /// Postado
  Posted,
  /// Waiting post
  Released,
}

/// Send freight response
#[derive(Debug, serde::Deserialize)]
pub struct SendFreightResGeneric<S> {
  /// Id
  pub id: S,
  /// Price
  #[serde(with = "rust_decimal::serde::float")]
  pub price: Decimal,
  /// Status
  pub status: SendFreightResStatus,
}
