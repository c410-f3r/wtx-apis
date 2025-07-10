use wtx::{
  calendar::{DateTime, DynTz},
  collection::Vector,
};

/// Status of a chargeback
#[derive(Debug, serde::Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ChargebackStatus {
  /// No received documentation
  Pending,
  /// Received documentation. Waiting for a decision
  ReviewPending,
  /// Documentation is valid
  Valid,
}

/// Chargeback
#[derive(Debug, serde::Deserialize)]
pub struct Chargeback<T> {
  /// Amount of the chargeback.
  pub amount: T,
  /// Indicates whether the seller is covered.
  pub coverage_applied: bool,
  /// Indicates if the chargeback can be covered by Mercado Pago.
  pub coverage_elegible: bool,
  /// Currency of the chargeback amount.
  pub currency: T,
  /// Creation date of the chargeback.
  pub date_created: DateTime<DynTz>,
  /// Last modification date of the chargeback.
  pub date_last_updated: DateTime<DynTz>,
  /// List of documentation received from the seller.
  pub documentation: Vector<ChargebackDocumentation<T>>,
  /// Indicates if the seller needs to send documentation for this chargeback.
  pub documentation_required: bool,
  /// Status of the documentation submitted.
  pub documentation_status: ChargebackStatus,
  /// Deadline for submitting chargeback documentation.
  pub date_documentation_deadline: T,
  /// Unique identifier for the chargeback.
  pub id: u64,
  /// Indicates if the chargeback will be processed in production mode or sandbox mode.
  pub live_mode: bool,
}

/// Documentation received from a seller
#[derive(Debug, serde::Deserialize)]
pub struct ChargebackDocumentation<T> {
  /// Description
  pub description: T,
  /// Type
  pub r#type: T,
  /// Url
  pub url: T,
}
