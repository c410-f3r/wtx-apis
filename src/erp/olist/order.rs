use serde::{Serialize, Serializer};

wtx::create_enum! {
  /// Represents the possible statuses of an order.
  #[derive(Debug, Clone, Copy, serde::Deserialize)]
  #[serde(try_from = "u8")]
  pub enum OrderStatus<u8> {
    /// Order is open and being processed
    Open = (0, "aberto"),
    /// Order has been invoiced/billed
    Invoiced = (1, "faturado"),
    /// Order has been canceled
    Canceled = (2, "cancelado"),
    /// Order has been approved for processing
    Approved = (3, "aprovado"),
    /// Order is being prepared for shipment
    PreparingShipment = (4, "preparando_envio"),
    /// Order has been shipped
    Shipped = (5, "enviado"),
    /// Order has been delivered to the customer
    Delivered = (6, "entregue"),
    /// Order is ready for shipment
    ReadyForShipment = (7, "pronto_envio"),
    /// Order could not be delivered
    NotDelivered = (8, "nao_entregue"),
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
