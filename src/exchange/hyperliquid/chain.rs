use crate::exchange::hyperliquid::Hyperliquid;

/// Network type
#[derive(Clone, Copy, Debug, PartialEq, serde::Deserialize, serde::Serialize)]
pub enum Chain {
  /// Mainnet
  #[serde(rename = "Mainnet")]
  Mainnet,
  /// Testnet
  #[serde(rename = "Testnet")]
  Testnet,
}

impl Chain {
  pub(crate) fn from_api(api: &Hyperliquid) -> Self {
    if api.is_mainnet { Self::Mainnet } else { Self::Testnet }
  }
}

impl From<Chain> for &'static str {
  fn from(value: Chain) -> Self {
    match value {
      Chain::Mainnet => "Mainnet",
      Chain::Testnet => "Testnet",
    }
  }
}
