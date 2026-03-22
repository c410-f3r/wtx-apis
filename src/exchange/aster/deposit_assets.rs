use crate::{AddressString, AssetString};
use wtx::collection::{ArrayStringU8, Vector};

/// Represents the query parameters for the Get All Deposit Assets endpoint
#[derive(Debug, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DepositAssetsReqParams<'any> {
  /// Chain ID, multiple IDs separated by commas
  pub chain_ids: &'any str,
  /// Network type, e.g., EVM, SOLANA, multiple networks separated by commas (Optional)
  #[serde(skip_serializing_if = "Option::is_none")]
  pub networks: Option<&'any str>,
  /// Account type, e.g., spot, perp
  pub account_type: &'any str,
}

/// Represents the top-level API response
#[derive(Debug, serde::Deserialize)]
pub struct DepositAssetsResParams {
  /// Response status code (e.g., "000000")
  pub code: ArrayStringU8<8>,
  /// List of deposit assets found
  pub data: Vector<DepositAssetsResData>,
}

/// Represents a specific asset definition within the data list
#[derive(Debug, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DepositAssetsResData {
  /// The ticker name of the asset (e.g., "ASTER")
  pub name: AssetString,
  /// The display name for the UI
  pub display_name: AssetString,
  /// The contract address on the blockchain
  pub contract_address: AddressString,
  /// Precision of the token (e.g., 18)
  pub decimals: u8,
  /// The network type (e.g., "EVM")
  pub network: ArrayStringU8<8>,
  /// The specific chain ID (e.g., 56)
  pub chain_id: u64,
  /// The type of deposit (e.g., "normal")
  pub deposit_type: ArrayStringU8<8>,
  /// sorting rank of the asset
  pub rank: i32,
  /// Whether the token is the native gas token for the chain
  pub is_native: bool,
}
