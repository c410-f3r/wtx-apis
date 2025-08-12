wtx::create_enum! {
  /// Tributary regime
  #[derive(Clone, Copy, Debug, PartialEq, serde::Deserialize, serde::Serialize)]
  #[serde(try_from = "u8")]
  pub enum TributaryRegime<u8> {
    /// Simples
    Simples = (1),
    /// Simples exceed in national treasury
    SimplesExceed = (2),
    /// Normal
    Normal = (3),
    /// Mei
    Mei = (4),
  }
}
