use rust_decimal::Decimal;
use wtx::misc::Vector;

/// Represents a shipping quote request containing origin, destination,
/// products, delivery options, and volume information.
#[derive(Debug, serde::Deserialize, serde::Serialize)]
pub struct CalculateShipmentRequest<S> {
  /// The origin location of the shipment.
  pub from: CalculateShipmentRequestPostalCode<S>,
  /// The destination location of the shipment.
  pub to: CalculateShipmentRequestPostalCode<S>,
  /// List of products contained in the shipment package.
  pub products: Option<Vector<CalculateShipmentRequestProduct<S>>>,
  /// Delivery service options for the shipment.
  pub options: Option<CalculateShipmentRequestOptions>,
  /// List of volume specifications for the shipment.
  pub volumes: Option<Vector<CalculateShipmentRequestVolume>>,
}

/// Represents delivery service options for the shipment.
#[derive(Debug, serde::Deserialize, serde::Serialize)]
pub struct CalculateShipmentRequestOptions {
  /// Requires that the indicated person receives the packages
  pub own_hand: bool,
  /// Confirms the receiving of the packages
  pub receipt: bool,
}

/// Represents a location (origin or destination) in the shipping quote.
#[derive(Debug, serde::Deserialize, serde::Serialize)]
pub struct CalculateShipmentRequestPostalCode<S> {
  /// Street address of the location.
  pub postal_code: S,
}

/// Represents a product in the shipment.
#[derive(Debug, serde::Deserialize, serde::Serialize)]
pub struct CalculateShipmentRequestProduct<S> {
  /// Unique identifier for the product.
  pub id: S,
  /// Width of the item in centimeters.
  pub width: Decimal,
  /// Height of the item in centimeters.
  pub height: Decimal,
  /// Length of the item in centimeters.
  pub length: Decimal,
  /// Weight of the product in kilograms.
  pub weight: Decimal,
  /// Insurance value
  pub insurance_value: Decimal,
  /// Number of units of this product in the shipment.
  pub quantity: Decimal,
}

/// Represents the physical volume characteristics of a package.
#[derive(Debug, serde::Deserialize, serde::Serialize)]
pub struct CalculateShipmentRequestVolume {
  /// Width of the package in centimeters.
  pub width: Decimal,
  /// Height of the package in centimeters.
  pub height: Decimal,
  /// Length of the package in centimeters.
  pub length: Decimal,
  /// Weight of the package in kilograms.
  pub weight: Decimal,
  /// Unit of measurement for dimensions.
  pub insurance_value: Decimal,
}
