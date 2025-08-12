use rust_decimal::Decimal;

/// Represents a freight shipment request.
#[derive(Debug, serde::Serialize)]
pub struct QuoteFreightReqGeneric<P, S> {
  /// Origin postal code.
  pub from: QuoteFreightReqPostalCode<S>,
  /// Optional freight service configurations.
  #[serde(skip_serializing_if = "Option::is_none")]
  pub options: Option<QuoteFreightReqOptions>,
  /// Optional package information for the freight.
  #[serde(skip_serializing_if = "Option::is_none")]
  pub package: Option<QuoteFreightReqPackage>,
  /// Optional products
  #[serde(skip_serializing_if = "Option::is_none")]
  pub products: Option<P>,
  /// Possible services separated by commas
  pub services: S,
  /// Destination postal code.
  pub to: QuoteFreightReqPostalCode<S>,
}

/// Options for configuring freight services.
#[derive(Debug, serde::Serialize)]
pub struct QuoteFreightReqOptions {
  /// Declared value for insurance purposes.
  #[serde(skip_serializing_if = "Option::is_none")]
  pub insurance_value: Option<Decimal>,
  /// Indicates if "own hand" freight service is requested.
  #[serde(skip_serializing_if = "Option::is_none")]
  pub own_hand: Option<bool>,
  /// Indicates if delivery receipt is required.
  #[serde(skip_serializing_if = "Option::is_none")]
  pub receipt: Option<bool>,
  /// Indicates if `insurance_value` should be used for insurance calculation.
  #[serde(skip_serializing_if = "Option::is_none")]
  pub use_insurance_value: Option<bool>,
}

/// Represents a package for freight calculation.
#[derive(Debug, serde::Serialize)]
pub struct QuoteFreightReqPackage {
  /// Height of the package.
  pub height: Decimal,
  /// Length of the package.
  pub length: Decimal,
  /// Weight of the package.
  pub weight: Decimal,
  /// Width of the package.
  pub width: Decimal,
}

/// Postal code for freight operations.
#[derive(Debug, serde::Serialize)]
pub struct QuoteFreightReqPostalCode<S> {
  /// Postal code string.
  pub postal_code: S,
}

/// Represents a product item for freight calculation.
#[derive(Debug, serde::Serialize)]
pub struct QuoteFreightReqProduct {
  /// Height of a single product item.
  pub height: Decimal,
  /// Length of a single product item.
  pub length: Decimal,
  /// Quantity of the product items.
  pub quantity: Decimal,
  /// Weight of a single product item.
  pub weight: Decimal,
  /// Width of a single product item.
  pub width: Decimal,
}
