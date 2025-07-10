use ethabi::Address;
use ethereum_types::H256;
use wtx::collection::Vector;

/// Access list item
#[derive(Debug, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AccessListItem {
  /// Accessed address
  pub address: Address,
  /// Accessed storage keys
  pub storage_keys: Vector<H256>,
}
