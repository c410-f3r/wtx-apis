use rust_decimal::Decimal;

/// Represents a freight shipment request.
#[derive(Debug, serde::Serialize)]
pub struct QuoteFreightReqGeneric<P, S> {
  /// Origin postal code.
  pub from: QuoteFreightReqPostalCode<S>,
  /// Optional freight service configurations.
  pub options: Option<QuoteFreightReqOptions>,
  /// Optional package information for the freight.
  pub package: Option<QuoteFreightReqPackage>,
  /// Optional products
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
  pub insurance_value: Option<Decimal>,
  /// Indicates if "own hand" freight service is requested.
  pub own_hand: Option<bool>,
  /// Indicates if delivery receipt is required.
  pub receipt: Option<bool>,
  /// Indicates if `insurance_value` should be used for insurance calculation.
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
