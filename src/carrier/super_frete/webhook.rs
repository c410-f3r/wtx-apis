use wtx::calendar::{DateTime, DynTz};

/// Webhook type
#[derive(Debug, serde::Deserialize, serde::Serialize)]
pub enum WebhookEvent {
  /// Order was canceled
  #[serde(rename = "order.cancelled")]
  Cancelled,
  /// Order was created
  #[serde(rename = "order.created")]
  Created,
  /// Order was delivered by the carrier
  #[serde(rename = "order.delivered")]
  Delivered,
  /// Order was generated
  #[serde(rename = "order.generated")]
  Generated,
  /// Order was posted to the carrier
  #[serde(rename = "order.posted")]
  Posted,
  /// Order was paid
  #[serde(rename = "order.released")]
  Released,
}

/// Webhook
#[derive(Debug, serde::Deserialize, serde::Serialize)]
pub struct Webhook<S> {
  /// See [`WebhookEvent`].
  pub event: WebhookEvent,
  /// See [`WebhookData`].
  pub data: WebhookData<S>,
}

/// Webhook data
#[derive(Debug, serde::Deserialize, serde::Serialize)]
pub struct WebhookData<S> {
  /// Canceled at
  pub canceled_at: Option<DateTime<DynTz>>,
  /// Created at
  pub created_at: Option<DateTime<DynTz>>,
  /// Delivered at
  pub delivered_at: Option<DateTime<DynTz>>,
  /// Expired at
  pub expired_at: Option<DateTime<DynTz>>,
  /// Generated at
  pub generated_at: Option<DateTime<DynTz>>,
  /// Id
  pub id: Option<S>,
  /// Paid at
  pub paid_at: Option<DateTime<DynTz>>,
  /// Posted at
  pub posted_at: Option<DateTime<DynTz>>,
  /// Protocol
  pub protocol: Option<S>,
  /// Self tracking
  pub self_tracking: Option<S>,
  /// Status
  pub status: Option<S>,
  /// TRacking URL
  pub tracking_url: Option<S>,
  /// Tracking
  pub tracking: Option<S>,
  /// User ID
  pub user_id: Option<S>,
}
