use rust_decimal::Decimal;

/// Send freight response
#[derive(Debug, serde::Serialize)]
pub struct SendFreightReqAddress<S> {
  /// Address
  pub address: S,
  /// City
  pub city: S,
  /// Complement
  pub complement: Option<S>,
  /// District
  pub district: S,
  /// Name
  pub name: S,
  /// Number
  pub number: S,
  /// Postal code
  pub postal_code: S,
  /// State abbreviation
  pub state_abbr: S,
}

/// Send freight response to
#[derive(Debug, serde::Serialize)]
pub struct SendFreightReqAddressTo<S> {
  /// Address
  #[serde(flatten)]
  pub address: SendFreightReqAddress<S>,
  /// Email
  pub email: S,
}

/// Represents dimensions with height, length, and width.
#[derive(Debug, serde::Deserialize, serde::Serialize)]
pub struct SendFreightReqVolumes {
  /// Height
  pub height: Decimal,
  /// Length
  pub length: Decimal,
  /// Weight
  pub weight: Decimal,
  /// Width
  pub width: Decimal,
}

/// Send freight response
#[derive(Debug, serde::Serialize)]
pub struct SendFreightReqGeneric<P, S> {
  /// From address
  pub from: SendFreightReqAddress<S>,
  /// Options
  pub options: SendFreightReqOptions<S>,
  /// Platform
  pub platform: Option<S>,
  /// Products
  pub products: Option<P>,
  /// Service ID
  pub service: u8,
  /// To address
  pub to: SendFreightReqAddressTo<S>,
  /// Volumes
  pub volumes: SendFreightReqVolumes,
}

/// Send freight options
#[derive(Debug, serde::Serialize)]
pub struct SendFreightReqOptions<S> {
  /// Declared value for insurance purposes.
  pub insurance_value: Option<Decimal>,
  /// Invoice
  pub invoice: Option<SendFreightReqOptionsInvoice<S>>,
  /// Is not comemercial
  pub non_commercial: Option<bool>,
  /// Indicates if "own hand" freight service is requested.
  pub own_hand: Option<bool>,
  /// Indicates if delivery receipt is required.
  pub receipt: Option<bool>,
}

/// Send freight options invoice
#[derive(Debug, serde::Serialize)]
pub struct SendFreightReqOptionsInvoice<S> {
  /// Key
  pub key: S,
  /// Number
  pub number: S,
}

/// Send freight product
#[derive(Debug, serde::Serialize)]
pub struct SendFreightReqProduct<S> {
  /// Name
  pub name: S,
  /// Quantity
  #[serde(with = "rust_decimal::serde::str")]
  pub quantity: Decimal,
  /// Unitary value
  #[serde(with = "rust_decimal::serde::str")]
  pub unitary_value: Decimal,
}
