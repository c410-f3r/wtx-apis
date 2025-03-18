use crate::carrier::super_frete::PackageFormat;
use rust_decimal::Decimal;

/// Represents additional services requested for the freight.
#[derive(Debug, serde::Deserialize)]
pub struct QuoteFreightResAdditionalServices {
  /// Indicates if own hand delivery is requested.
  pub own_hand: bool,
  /// Indicates if receipt confirmation is requested.
  pub receipt: bool,
}

/// Represents company information.
#[derive(Debug, serde::Deserialize)]
pub struct QuoteFreightResCompany<S> {
  /// Unique identifier for the company.
  pub id: u32,
  /// Name of the company.
  pub name: S,
  /// Picture URL of the company.
  pub picture: S,
}

/// Represents a delivery range with minimum and maximum values.
#[derive(Debug, serde::Deserialize)]
pub struct QuoteFreightResDeliveryRange {
  /// Maximum delivery range.
  pub max: u8,
  /// Minimum delivery range.
  pub min: u8,
}

/// Represents dimensions with height, length, and width.
#[derive(Debug, serde::Deserialize)]
pub struct QuoteFreightResDimensions {
  /// Height of the dimension.
  #[serde(with = "rust_decimal::serde::str")]
  pub height: Decimal,
  /// Length of the dimension.
  #[serde(with = "rust_decimal::serde::str")]
  pub length: Decimal,
  /// Width of the dimension.
  #[serde(with = "rust_decimal::serde::str")]
  pub width: Decimal,
}

/// Represents freight information.
#[derive(Debug, serde::Deserialize)]
pub struct QuoteFreightResGeneric<P, S> {
  /// Additional services for the freight.
  pub additional_services: QuoteFreightResAdditionalServices,
  /// Company information for the freight.
  pub company: QuoteFreightResCompany<S>,
  /// Currency of the freight price.
  pub currency: S,
  /// Delivery range for the freight.
  pub delivery_range: QuoteFreightResDeliveryRange,
  /// Delivery time in days for the freight.
  pub delivery_time: u8,
  /// Discount applied to the freight.
  #[serde(with = "rust_decimal::serde::str")]
  pub discount: Decimal,
  /// Indicates if there is an error with the freight.
  pub has_error: bool,
  /// Unique identifier for the freight.
  pub id: u8,
  /// Name of the freight.
  pub name: S,
  /// Packages associated with the freight.
  pub packages: P,
  /// Price of the freight.
  #[serde(with = "rust_decimal::serde::float")]
  pub price: Decimal,
}

/// Represents a package with its dimensions, weight, price, and other details.
#[derive(Debug, serde::Deserialize)]
pub struct QuoteFreightResPackage {
  /// Dimensions of the package.
  pub dimensions: QuoteFreightResDimensions,
  /// Discount applied to the package.
  #[serde(with = "rust_decimal::serde::str")]
  pub discount: Decimal,
  /// Format of the package.
  pub format: PackageFormat,
  /// Insurance value of the package.
  #[serde(with = "rust_decimal::serde::float")]
  pub insurance_value: Decimal,
  /// Price of the package.
  #[serde(with = "rust_decimal::serde::float")]
  pub price: Decimal,
  /// Weight of the package.
  #[serde(with = "rust_decimal::serde::str")]
  pub weight: Decimal,
}

#[cfg(test)]
mod tests {
  use crate::carrier::super_frete::QuoteFreightRes;

  #[test]
  fn json() {
    let _elem = serde_json::from_str::<QuoteFreightRes<'_>>(include_str!(
      "../../../assets/super_frete/quote_freight_response.json"
    ))
    .unwrap();
  }
}
