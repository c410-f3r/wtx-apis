use rust_decimal::Decimal;
use wtx::{
  calendar::{DateTime, DynTz},
  collection::Vector,
};

/// Response's result
#[derive(Debug, serde::Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RefundStatus {
  /// Approved
  Approved,
  /// In process
  InProcess,
  /// Rejected
  Rejected,
}

/// Refund
#[derive(Debug, serde::Deserialize)]
pub struct Refund<T> {
  /// Adjustment amount
  #[serde(with = "rust_decimal::serde::float")]
  pub adjustment_amount: Decimal,
  /// Amount
  #[serde(with = "rust_decimal::serde::float")]
  pub amount: Decimal,
  /// Date created
  pub date_created: DateTime<DynTz>,
  /// Id
  pub id: u64,
  /// Payment Id
  pub payment_id: u64,
  /// Reason
  pub reason: T,
  /// Refund mode
  pub refund_mode: T,
  /// Sources
  pub source: Vector<RefundSource<T>>,
  /// Status
  pub status: RefundStatus,
  /// Unique sequence number
  pub unique_sequence_number: T,
}

/// Source of a refund
#[derive(Debug, serde::Deserialize)]
pub struct RefundSource<T> {
  /// Id
  pub id: T,
  /// Name
  pub name: T,
  /// Type
  pub r#type: T,
}
