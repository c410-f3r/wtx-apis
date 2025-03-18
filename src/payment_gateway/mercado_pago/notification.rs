use chrono::{DateTime, Utc};

/// WebHook notification
#[derive(Debug, Eq, PartialEq, serde::Deserialize)]
pub struct Notification {
  /// See [NotificationAction].
  pub action: NotificationAction,
  /// See [ApiVersion].
  pub api_version: ApiVersion,
  /// See [NotificationData].
  pub data: NotificationData,
  /// Date created
  pub date_created: DateTime<Utc>,
  /// Id
  pub id: u64,
  /// Live mode
  pub live_mode: bool,
  /// See [NotificationTy].
  pub r#type: NotificationTy,
  /// User id
  pub user_id: u64,
}

/// Api version
#[derive(Debug, Eq, PartialEq, serde::Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ApiVersion {
  /// V1
  V1,
}

/// Notification action
#[derive(Debug, Eq, PartialEq, serde::Deserialize)]
pub enum NotificationAction {
  /// Authorized application
  #[serde(rename = "application.deauthorized")]
  ApplicationAuthorized,
  /// Deauthorized application
  #[serde(rename = "application.authorized")]
  ApplicationDeauthorized,
  /// Created payment
  #[serde(rename = "payment.created")]
  PaymentCreated,
  /// Updated payment
  #[serde(rename = "payment.updated")]
  PaymentUpdated,
}

/// Notification type
#[derive(Debug, Eq, PartialEq, serde::Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum NotificationTy {
  /// Payment
  Payment,
}

/// Notification data
#[derive(Debug, Eq, PartialEq, serde::Deserialize)]
pub struct NotificationData {
  /// Id
  pub id: u64,
}

#[cfg(test)]
mod tests {
  use crate::payment_gateway::mercado_pago::{
    ApiVersion, Notification, NotificationAction, NotificationTy,
  };
  use chrono::DateTime;

  #[test]
  fn json() {
    {
      let notification: Notification = serde_json::from_str(include_str!(
        "../../../assets/mercado_pago/response/notification/payment_created.json"
      ))
      .unwrap();
      assert_eq!(notification.action, NotificationAction::PaymentCreated);
      assert_eq!(notification.api_version, ApiVersion::V1);
      assert_eq!(notification.data.id, 7547658345);
      assert_eq!(
        notification.date_created,
        DateTime::parse_from_rfc3339("2025-01-14T19:19:25Z").unwrap()
      );
      assert_eq!(notification.id, 636490643061);
      assert_eq!(notification.live_mode, false);
      assert_eq!(notification.r#type, NotificationTy::Payment);
      assert_eq!(notification.user_id, 5437642344);
    }

    {
      let notification: Notification = serde_json::from_str(include_str!(
        "../../../assets/mercado_pago/response/notification/payment_updated.json"
      ))
      .unwrap();
      assert_eq!(notification.action, NotificationAction::PaymentUpdated);
      assert_eq!(notification.api_version, ApiVersion::V1);
      assert_eq!(notification.data.id, 123456);
      assert_eq!(
        notification.date_created,
        DateTime::parse_from_rfc3339("2021-11-01T02:02:02Z").unwrap()
      );
      assert_eq!(notification.id, 123456);
      assert_eq!(notification.live_mode, false);
      assert_eq!(notification.r#type, NotificationTy::Payment);
      assert_eq!(notification.user_id, 7657454343);
    }
  }
}
