use serde::{Serialize, Serializer};

wtx::create_enum! {
  /// Represents the possible statuses of an order.
  #[derive(Debug, Clone, Copy, serde::Deserialize)]
  #[serde(try_from = "u8")]
  pub enum OrderStatus<u8> {
    /// Order is open and being processed
    Open = (0),
    /// Order has been invoiced/billed
    Invoiced = (1),
    /// Order has been canceled
    Canceled = (2),
    /// Order has been approved for processing
    Approved = (3),
    /// Order is being prepared for shipment
    PreparingShipment = (4),
    /// Order has been shipped
    Shipped = (5),
    /// Order has been delivered to the customer
    Delivered = (6),
    /// Order is ready for shipment
    ReadyForShipment = (7),
    /// Order has incomplete or missing data
    IncompleteData = (8),
    /// Order could not be delivered
    NotDelivered = (9),
  }
}

impl Serialize for OrderStatus {
  fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
  where
    S: Serializer,
  {
    serializer.serialize_u8(u8::from(*self))
  }
}
