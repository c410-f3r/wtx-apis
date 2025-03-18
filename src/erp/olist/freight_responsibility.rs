/// Freight Responsibility
#[derive(Clone, Copy, Debug, serde::Deserialize, serde::Serialize)]
pub enum FreightResponsibility {
  /// Freight contracted by the sender (CIF)
  #[serde(rename = "R")]
  BySender,
  /// Freight contracted by the recipient (FOB)
  #[serde(rename = "D")]
  ByRecipient,
  /// Freight contracted by a third party
  #[serde(rename = "T")]
  ByThirdParty,
  /// Transportation carried out by the sender
  #[serde(rename = "3")]
  SenderOwnTransportation,
  /// Transportation carried out by the recipient
  #[serde(rename = "4")]
  RecipientOwnTransportation,
  /// No transportation involved
  #[serde(rename = "S")]
  NoTransportation,
}
