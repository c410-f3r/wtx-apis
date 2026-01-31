use crate::exchange::extended::StarknetDomain;

#[derive(Debug)]
pub struct EndpointConfig {
  pub api_base_url: &'static str,
  pub asset_operations_contract: &'static str,
  pub chain_rpc_url: &'static str,
  pub collateral_asset_contract: &'static str,
  pub collateral_asset_id: &'static str,
  pub collateral_asset_on_chain_id: &'static str,
  pub collateral_decimals: u8,
  pub onboarding_url: &'static str,
  pub signing_domain: &'static str,
  pub starknet_domain: StarknetDomain<&'static str>,
  pub stream_url: &'static str,
}

impl EndpointConfig {
  pub const MAINNET_CONFIG: Self = Self {
    api_base_url: "https://api.starknet.extended.exchange/api",
    asset_operations_contract: "",
    chain_rpc_url: "",
    collateral_asset_contract: "",
    collateral_asset_id: "0x1",
    collateral_asset_on_chain_id: "0x1",
    collateral_decimals: 6,
    onboarding_url: "https://api.starknet.extended.exchange",
    signing_domain: "extended.exchange",
    starknet_domain: StarknetDomain {
      chain_id: "SN_MAIN",
      name: "Perpetuals",
      revision: 1,
      version: "v0",
    },
    stream_url: "wss://api.starknet.extended.exchange/stream.extended.exchange",
  };

  pub const TESTNET_CONFIG: Self = Self {
    api_base_url: "https://api.starknet.sepolia.extended.exchange/api",
    asset_operations_contract: "",
    chain_rpc_url: "https://rpc.sepolia.org",
    collateral_asset_contract: "",
    collateral_asset_id: "0x1",
    collateral_asset_on_chain_id: "",
    collateral_decimals: 6,
    onboarding_url: "https://api.starknet.sepolia.extended.exchange",
    signing_domain: "starknet.sepolia.extended.exchange",
    starknet_domain: StarknetDomain {
      name: "Perpetuals",
      version: "v0",
      chain_id: "SN_SEPOLIA",
      revision: 1,
    },
    stream_url: "wss://api.starknet.sepolia.extended.exchange/stream.extended.exchange",
  };
}
