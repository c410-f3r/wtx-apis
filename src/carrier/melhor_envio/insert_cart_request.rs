use rust_decimal::Decimal;
use wtx::misc::Vector;

/// Represents a shipment request with all necessary information for transportation
#[derive(Debug, serde::Deserialize, serde::Serialize)]
pub struct InsertCartRequest<S> {
  /// ID referencing the carrier's service
  pub service: u32,
  /// ID of the agency/unit where the shipment will be posted
  /// (consult carrier's rules)
  pub agency: Option<u32>,
  /// Sender's information
  pub from: InsertCartRequestAddress<S>,
  /// Recipient's information
  pub to: InsertCartRequestAddress<S>,
  /// List of products to be shipped (used for content declaration)
  pub products: Vector<InsertCartRequestShipmentProduct<S>>,
  /// List of volumes contained in the shipment
  pub volumes: Vector<InsertCartRequestVolume>,
  /// Options
  pub options: Option<InsertCartRequestOptions<S>>,
}

/// Contains address and contact information for sender or recipient
#[derive(Debug, serde::Deserialize, serde::Serialize)]
pub struct InsertCartRequestAddress<S> {
  /// Full name
  pub name: Option<S>,
  /// Phone number
  pub phone: Option<S>,
  /// Email address
  pub email: Option<S>,
  /// CPF (individual taxpayer ID)
  pub document: Option<S>,
  /// CNPJ (company taxpayer ID)
  pub company_document: Option<S>,
  /// State registration
  pub state_register: Option<S>,
  /// Street address
  pub address: Option<S>,
  /// Address complement
  pub complement: Option<S>,
  /// Address number
  pub number: Option<S>,
  /// Neighborhood
  pub district: Option<S>,
  /// City
  pub city: Option<S>,
  /// Country ID
  pub country_id: Option<S>,
  /// Postal code (CEP)
  pub postal_code: S,
  /// State abbreviation
  pub state_abbr: Option<S>,
  /// Additional notes
  pub note: Option<S>,
}

/// Struct representing additional shipping information and options
#[derive(Debug, serde::Deserialize, serde::Serialize)]
pub struct InsertCartRequestOptions<S> {
  /// Insured value for the shipment
  pub insurance_value: Decimal,
  /// Flag to enable Delivery Receipt additional service
  pub receipt: bool,
  /// Flag to enable Own Hand delivery additional service
  pub own_hand: bool,
  /// Flag to identify the shipment as reverse logistics
  pub reverse: bool,
  /// Flag to identify the shipment as non-commercial
  pub non_commercial: bool,
  /// Invoice information for the shipment
  pub invoice: Option<InsertCartRequestOptionsInvoice<S>>,
  /// Platform identifier for the shipment
  pub platform: Option<S>,
  /// Collection of tags associated with the shipment
  pub tags: Option<Vector<InsertCartRequestOptionsTag<S>>>,
}

/// Struct representing invoice information
#[derive(Debug, serde::Deserialize, serde::Serialize)]
pub struct InsertCartRequestOptionsInvoice<S> {
  /// Invoice key
  pub key: S,
}

/// Struct representing a tag for the shipment
#[derive(Debug, serde::Deserialize, serde::Serialize)]
pub struct InsertCartRequestOptionsTag<S> {
  /// Platform's identification
  pub tag: Option<S>,
  /// Platform's URL
  pub url: Option<S>,
}

/// Represents a product in the shipment
#[derive(Debug, serde::Deserialize, serde::Serialize)]
pub struct InsertCartRequestShipmentProduct<S> {
  /// Product name
  pub name: S,
  /// Quantity of the product
  pub quantity: Option<Decimal>,
  /// Unitary value of the product
  pub unitary_value: Option<S>,
}

/// Represents a volume (package) in the shipment
#[derive(Debug, serde::Deserialize, serde::Serialize)]
pub struct InsertCartRequestVolume {
  /// Height in centimeters
  pub height: Decimal,
  /// Width in centimeters
  pub width: Decimal,
  /// Length in centimeters
  pub length: Decimal,
  /// Weight in kilograms
  pub weight: Decimal,
}
