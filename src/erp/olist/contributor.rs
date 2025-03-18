wtx::create_enum! {
  /// Contributor
  #[derive(Clone, Copy, Debug, PartialEq, serde::Deserialize, serde::Serialize)]
  #[serde(try_from = "u8")]
  pub enum Contributor<u8> {
    /// Unknown
    Unknown = (0),
    /// ICMS
    Icms = (1),
    /// IE Exempt
    IeExempt = (2),
    /// No ICMS
    NoIcms = (9),
  }
}
