use rust_decimal::Decimal;
use wtx::collection::Vector;

/// Represents a shipment with detailed information about pricing, delivery, and contents
#[derive(Debug, serde::Deserialize, serde::Serialize)]
pub struct CalculateShipmentResponse<S> {
  /// Unique identifier for the shipment
  /// Defaults to 0
  pub id: u32,
  /// Name or description of the shipment
  pub name: S,
  /// Base price of the shipment
  pub price: S,
  /// Custom price for the shipment, if applicable
  pub custom_price: S,
  /// Discount applied to the shipment
  pub discount: S,
  /// Currency used for pricing
  pub currency: S,
  /// Standard delivery time in days
  pub delivery_time: u32,
  /// Standard delivery time range
  pub delivery_range: CalculateShipmentResponseDeliveryRange,
  /// Custom delivery time in days, if applicable
  pub custom_delivery_time: u32,
  /// Custom delivery time range, if applicable
  pub custom_delivery_range: CalculateShipmentResponseDeliveryRange,
  /// List of packages included in the shipment
  pub packages: Vector<CalculateShipmentResponsePackage<S>>,
  /// Additional services requested for the shipment
  pub additional_services: CalculateShipmentResponseAdditionalServices,
  /// Company information associated with the shipment
  pub company: CalculateShipmentResponseCompany<S>,
}

/// Represents additional services for a shipment
#[derive(Debug, serde::Deserialize, serde::Serialize)]
pub struct CalculateShipmentResponseAdditionalServices {
  /// Whether a receipt is required
  pub receipt: bool,
  /// Whether own-hand delivery is required
  pub own_hand: bool,
  /// Whether collection is required
  pub collect: bool,
}

/// Represents company information associated with a shipment
#[derive(Debug, serde::Deserialize, serde::Serialize)]
pub struct CalculateShipmentResponseCompany<S> {
  /// Unique identifier for the company
  pub id: u32,
  /// Name of the company
  pub name: S,
  /// URL or path to the company logo
  pub picture: S,
}

/// Range in days of a shipment
#[derive(Debug, serde::Deserialize, serde::Serialize)]
pub struct CalculateShipmentResponseDeliveryRange {
  /// Minimum delivery time in days
  pub min: u32,
  /// Maximum delivery time in days
  pub max: u32,
}

/// Represents the physical dimensions of a package
#[derive(Debug, serde::Deserialize, serde::Serialize)]
pub struct CalculateShipmentResponseDimensions {
  /// Height of the package
  pub height: Decimal,
  /// Width of the package
  pub width: Decimal,
  /// Length of the package
  pub length: Decimal,
}

/// Represents a package within a shipment
#[derive(Debug, serde::Deserialize, serde::Serialize)]
pub struct CalculateShipmentResponsePackage<S> {
  /// Price of the package
  pub price: S,
  /// Discount applied to the package
  pub discount: S,
  /// Format or type of the package
  pub format: S,
  /// Physical dimensions of the package
  pub dimensions: CalculateShipmentResponseDimensions,
  /// Weight of the package
  pub weight: S,
  /// Insurance value for the package
  pub insurance_value: S,
}
