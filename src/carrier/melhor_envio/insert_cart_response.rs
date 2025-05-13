use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use wtx::collection::Vector;

/// Represents a shipment with all relevant details and associated data
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct InsertCartResponse<S> {
  /// Unique identifier for the shipment
  pub id: S,
  /// Protocol identifier for the shipment
  pub protocol: S,
  /// Service identifier
  pub service_id: u32,
  /// Agency identifier
  pub agency_id: u32,
  /// Contract identifier
  pub contract: S,
  /// Service code
  pub service_code: S,
  /// Quote amount
  pub quote: f64,
  /// Price amount
  pub price: f64,
  /// Coupon code
  pub coupon: S,
  /// Discount amount
  pub discount: f64,
  /// Minimum delivery time in days
  pub delivery_min: u32,
  /// Maximum delivery time in days
  pub delivery_max: u32,
  /// Current status of the shipment
  pub status: S,
  /// Reminder information
  pub reminder: S,
  /// Insurance value for the shipment
  pub insurance_value: Decimal,
  /// Weight of the shipment
  pub weight: S,
  /// Width of the shipment
  pub width: S,
  /// Height of the shipment
  pub height: S,
  /// Length of the shipment
  pub length: S,
  /// Diameter of the shipment
  pub diameter: S,
  /// Format of the shipment
  pub format: S,
  /// Billed weight
  pub billed_weight: f64,
  /// Indicates if receipt is required
  pub receipt: bool,
  /// Indicates if own hand delivery is required
  pub own_hand: bool,
  /// Indicates if collection is required
  pub collect: bool,
  /// Scheduled collection date and time
  pub collect_scheduled_at: S,
  /// Indicates if this is a reverse shipment
  pub reverse: bool,
  /// Indicates if this is a non-commercial shipment
  pub non_commercial: bool,
  /// Authorization code for the shipment
  pub authorization_code: S,
  /// Tracking code
  pub tracking: S,
  /// Self-tracking information
  pub self_tracking: S,
  /// Delivery receipt information
  pub delivery_receipt: S,
  /// Additional information about the shipment
  pub additional_info: S,
  /// CTE key
  pub cte_key: S,
  /// Date and time when payment was made
  pub paid_at: S,
  /// Date and time when shipment was generated
  pub generated_at: S,
  /// Date and time when shipment was posted
  pub posted_at: S,
  /// Date and time when shipment was delivered
  pub delivered_at: S,
  /// Date and time when shipment was canceled
  pub canceled_at: S,
  /// Date and time when shipment was suspended
  pub suspended_at: S,
  /// Date and time when shipment expired
  pub expired_at: S,
  /// Date and time when shipment was created
  pub created_at: S,
  /// Date and time when shipment was last updated
  pub updated_at: S,
  /// Date and time for parse PI
  pub parse_pi_at: S,
  /// List of products in the shipment
  pub products: Vector<InsertCartResponseProduct<S>>,
  /// List of volumes in the shipment
  pub volumes: Vector<InsertCartResponseVolume<S>>,
}

/// Represents a product in the shipment
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct InsertCartResponseProduct<S> {
  /// Product name
  pub name: S,
  /// Quantity of the product
  pub quantity: Decimal,
  /// Unitary value of the product
  pub unitary_value: Decimal,
  /// Weight of the product
  pub weight: S,
}

/// Represents a volume in the shipment
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct InsertCartResponseVolume<S> {
  /// Volume identifier
  pub id: u32,
  /// Height of the volume
  pub height: S,
  /// Width of the volume
  pub width: S,
  /// Length of the volume
  pub length: S,
  /// Diameter of the volume
  pub diameter: S,
  /// Weight of the volume
  pub weight: S,
  /// Format of the volume
  pub format: S,
  /// Date and time when volume was created
  pub created_at: S,
  /// Date and time when volume was last updated
  pub updated_at: S,
}
