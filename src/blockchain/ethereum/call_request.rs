use crate::blockchain::ethereum::{AccessList, Bytes};
use ethabi::Address;
use ethereum_types::{U64, U256};

/// Call contract request (eth_call / eth_estimateGas)
///
/// When using this for `eth_estimateGas`, all the fields
/// are optional. However, for usage in `eth_call` the
/// `to` field must be provided.
#[derive(Debug, Default, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CallRequest {
  /// Access list
  #[serde(default, skip_serializing_if = "Option::is_none")]
  pub access_list: Option<AccessList>,
  /// Data (None for empty data)
  #[serde(skip_serializing_if = "Option::is_none")]
  pub data: Option<Bytes>,
  /// Sender address (None for arbitrary address)
  #[serde(skip_serializing_if = "Option::is_none")]
  pub from: Option<Address>,
  /// Supplied gas (None for sensible default)
  #[serde(skip_serializing_if = "Option::is_none")]
  pub gas: Option<U256>,
  /// Gas price (None for sensible default)
  #[serde(skip_serializing_if = "Option::is_none")]
  pub gas_price: Option<U256>,
  /// Max fee per gas
  #[serde(default, skip_serializing_if = "Option::is_none")]
  pub max_fee_per_gas: Option<U256>,
  /// miner bribe
  #[serde(default, skip_serializing_if = "Option::is_none")]
  pub max_priority_fee_per_gas: Option<U256>,
  /// To address (None allowed for eth_estimateGas)
  #[serde(skip_serializing_if = "Option::is_none")]
  pub to: Option<Address>,
  /// Transaction type, Some(1) for AccessList transaction, None for Legacy
  #[serde(default, rename = "type", skip_serializing_if = "Option::is_none")]
  pub ty: Option<U64>,
  /// Transfered value (None for no transfer)
  #[serde(skip_serializing_if = "Option::is_none")]
  pub value: Option<U256>,
}

#[cfg(test)]
mod tests {
  use crate::blockchain::ethereum::CallRequest;
  use ethereum_types::Address;

  #[test]
  fn should_serialize_call_request() {
    let call_request = CallRequest {
      access_list: None,
      data: Some(hex::decode("010203").unwrap().into()),
      from: None,
      gas_price: None,
      gas: Some(21_000.into()),
      max_fee_per_gas: None,
      max_priority_fee_per_gas: None,
      to: Some(Address::from_low_u64_be(5)),
      ty: None,
      value: Some(5_000_000.into()),
    };
    assert_eq!(
      serde_json::to_string_pretty(&call_request).unwrap(),
      r#"{
  "data": "0x010203",
  "gas": "0x5208",
  "to": "0x0000000000000000000000000000000000000005",
  "value": "0x4c4b40"
}"#
    );
  }

  #[test]
  fn should_deserialize_call_request() {
    let serialized = r#"{
  "to": "0x0000000000000000000000000000000000000005",
  "gas": "0x5208",
  "value": "0x4c4b40",
  "data": "0x010203"
}"#;
    let deserialized: CallRequest = serde_json::from_str(&serialized).unwrap();

    assert_eq!(deserialized.from, None);
    assert_eq!(deserialized.to, Some(Address::from_low_u64_be(5)));
    assert_eq!(deserialized.gas, Some(21_000.into()));
    assert_eq!(deserialized.gas_price, None);
    assert_eq!(deserialized.value, Some(5_000_000.into()));
    assert_eq!(deserialized.data, Some(hex::decode("010203").unwrap().into()));
  }
}
