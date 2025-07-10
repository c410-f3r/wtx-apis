use crate::blockchain::ethereum::{AccessList, Bytes, TransactionCondition};
use ethabi::Address;
use ethereum_types::{U64, U256};

/// Request parameters when sending.
#[derive(Debug, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct TransactionRequest {
  /// Access list
  #[serde(default, skip_serializing_if = "Option::is_none")]
  pub access_list: Option<AccessList>,
  /// Min block inclusion (None for include immediately)
  #[serde(skip_serializing_if = "Option::is_none")]
  pub condition: Option<TransactionCondition>,
  /// Transaction data (None for empty bytes)
  #[serde(skip_serializing_if = "Option::is_none")]
  pub data: Option<Bytes>,
  /// Sender address
  pub from: Address,
  /// Supplied gas (None for sensible default)
  #[serde(skip_serializing_if = "Option::is_none")]
  pub gas: Option<U256>,
  /// Gas price (None for sensible default)
  #[serde(skip_serializing_if = "Option::is_none")]
  pub gas_price: Option<U256>,
  /// Max fee per gas
  #[serde(skip_serializing_if = "Option::is_none")]
  pub max_fee_per_gas: Option<U256>,
  /// miner bribe
  #[serde(skip_serializing_if = "Option::is_none")]
  pub max_priority_fee_per_gas: Option<U256>,
  /// Transaction nonce (None for next available nonce)
  #[serde(skip_serializing_if = "Option::is_none")]
  pub nonce: Option<U256>,
  /// Recipient address (None for contract creation)
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
  use crate::blockchain::ethereum::{TransactionCondition, TransactionRequest};
  use ethereum_types::Address;
  use wtx::de::decode_hex_to_slice;

  #[test]
  fn should_serialize_transaction_request() {
    let tx_request = TransactionRequest {
      from: Address::from_low_u64_be(5),
      to: None,
      gas: Some(21_000.into()),
      gas_price: None,
      value: Some(5_000_000.into()),
      data: Some(decode_hex_to_slice::<false>(b"010203", &mut [0; 8]).unwrap().to_vec().into()),
      nonce: None,
      condition: Some(TransactionCondition::Block(5)),
      ty: None,
      access_list: None,
      max_fee_per_gas: None,
      max_priority_fee_per_gas: None,
    };

    assert_eq!(
      serde_json::to_string_pretty(&tx_request).unwrap(),
      r#"{
  "condition": {
    "block": 5
  },
  "data": "0x010203",
  "from": "0x0000000000000000000000000000000000000005",
  "gas": "0x5208",
  "value": "0x4c4b40"
}"#
    );
  }

  #[test]
  fn should_deserialize_transaction_request() {
    let serialized = r#"{
  "from": "0x0000000000000000000000000000000000000005",
  "gas": "0x5208",
  "value": "0x4c4b40",
  "data": "0x010203",
  "condition": {
    "block": 5
  }
}"#;
    let deserialized: TransactionRequest = serde_json::from_str(&serialized).unwrap();

    assert_eq!(deserialized.from, Address::from_low_u64_be(5));
    assert_eq!(deserialized.to, None);
    assert_eq!(deserialized.gas, Some(21_000.into()));
    assert_eq!(deserialized.gas_price, None);
    assert_eq!(deserialized.value, Some(5_000_000.into()));
    assert_eq!(
      deserialized.data,
      Some(decode_hex_to_slice::<false>(b"010203", &mut [0; 8]).unwrap().to_vec().into())
    );
    assert_eq!(deserialized.nonce, None);
    assert_eq!(deserialized.condition, Some(TransactionCondition::Block(5)));
  }
}
