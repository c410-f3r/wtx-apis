use crate::blockchain::ethereum::{AccessList, Bytes};
use ethabi::Address;
use ethereum_types::{H256, U64, U256};

/// Description of a Transaction, pending or in the chain.
#[derive(Debug, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Transaction {
  /// Hash
  pub hash: H256,
  /// Nonce
  pub nonce: U256,
  /// Block hash. None when pending.
  pub block_hash: Option<H256>,
  /// Block number. None when pending.
  pub block_number: Option<U64>,
  /// Transaction Index. None when pending.
  pub transaction_index: Option<U64>,
  /// Sender
  #[serde(default, skip_serializing_if = "Option::is_none")]
  pub from: Option<Address>,
  /// Recipient (None when contract creation)
  pub to: Option<Address>,
  /// Transfered value
  pub value: U256,
  /// Gas Price
  pub gas_price: Option<U256>,
  /// Gas amount
  pub gas: U256,
  /// Input data
  pub input: Bytes,
  /// ECDSA recovery id
  #[serde(default, skip_serializing_if = "Option::is_none")]
  pub v: Option<U64>,
  /// ECDSA signature r, 32 bytes
  #[serde(default, skip_serializing_if = "Option::is_none")]
  pub r: Option<U256>,
  /// ECDSA signature s, 32 bytes
  #[serde(default, skip_serializing_if = "Option::is_none")]
  pub s: Option<U256>,
  /// Raw transaction data
  #[serde(default, skip_serializing_if = "Option::is_none")]
  pub raw: Option<Bytes>,
  /// Transaction type, Some(1) for AccessList transaction, None for Legacy
  #[serde(default, rename = "type", skip_serializing_if = "Option::is_none")]
  pub ty: Option<U64>,
  /// Access list
  #[serde(default, skip_serializing_if = "Option::is_none")]
  pub access_list: Option<AccessList>,
  /// Max fee per gas
  #[serde(skip_serializing_if = "Option::is_none")]
  pub max_fee_per_gas: Option<U256>,
  /// Miner bribe
  #[serde(skip_serializing_if = "Option::is_none")]
  pub max_priority_fee_per_gas: Option<U256>,
}

#[cfg(test)]
mod tests {
  use crate::blockchain::ethereum::{RawTransaction, Receipt};

  #[test]
  fn test_deserialize_receipt() {
    let receipt_str = "{\"blockHash\":\"0x83eaba432089a0bfe99e9fc9022d1cfcb78f95f407821be81737c84ae0b439c5\",\"blockNumber\":\"0x38\",\"contractAddress\":\"0x03d8c4566478a6e1bf75650248accce16a98509f\",\"from\":\"0x407d73d8a49eeb85d32cf465507dd71d507100c1\",\"to\":\"0x853f43d8a49eeb85d32cf465507dd71d507100c1\",\"cumulativeGasUsed\":\"0x927c0\",\"gasUsed\":\"0x927c0\",\"logs\":[],\"logsBloom\":\"0x00000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000\",\"root\":null,\"transactionHash\":\"0x422fb0d5953c0c48cbb42fb58e1c30f5e150441c68374d70ca7d4f191fd56f26\",\"transactionIndex\":\"0x0\",\"effectiveGasPrice\": \"0x100\"}";

    let _receipt: Receipt = serde_json::from_str(receipt_str).unwrap();
  }

  #[test]
  fn should_deserialize_receipt_without_from_to() {
    let receipt_str = r#"{
        "blockHash": "0x83eaba432089a0bfe99e9fc9022d1cfcb78f95f407821be81737c84ae0b439c5",
        "blockNumber": "0x38",
        "contractAddress": "0x03d8c4566478a6e1bf75650248accce16a98509f",
        "cumulativeGasUsed": "0x927c0",
        "gasUsed": "0x927c0",
        "logs": [],
        "logsBloom": "0x00000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000",
        "root": null,
        "transactionHash": "0x422fb0d5953c0c48cbb42fb58e1c30f5e150441c68374d70ca7d4f191fd56f26",
        "transactionIndex": "0x0",
        "status": "0x1",
        "effectiveGasPrice": "0x100"
        }"#;

    let _receipt: Receipt = serde_json::from_str(receipt_str).unwrap();
  }

  #[test]
  fn should_deserialize_receipt_with_status() {
    let receipt_str = r#"{
        "blockHash": "0x83eaba432089a0bfe99e9fc9022d1cfcb78f95f407821be81737c84ae0b439c5",
        "blockNumber": "0x38",
        "contractAddress": "0x03d8c4566478a6e1bf75650248accce16a98509f",
        "from": "0x407d73d8a49eeb85d32cf465507dd71d507100c1",
        "to": "0x853f43d8a49eeb85d32cf465507dd71d507100c1",
        "cumulativeGasUsed": "0x927c0",
        "gasUsed": "0x927c0",
        "logs": [],
        "logsBloom": "0x00000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000",
        "root": null,
        "transactionHash": "0x422fb0d5953c0c48cbb42fb58e1c30f5e150441c68374d70ca7d4f191fd56f26",
        "transactionIndex": "0x0",
        "status": "0x1",
        "effectiveGasPrice": "0x100"
    }"#;

    let _receipt: Receipt = serde_json::from_str(receipt_str).unwrap();
  }

  #[test]
  fn should_deserialize_receipt_without_to() {
    let receipt_str = r#"{
        "blockHash": "0x83eaba432089a0bfe99e9fc9022d1cfcb78f95f407821be81737c84ae0b439c5",
        "blockNumber": "0x38",
        "contractAddress": "0x03d8c4566478a6e1bf75650248accce16a98509f",
        "from": "0x407d73d8a49eeb85d32cf465507dd71d507100c1",
        "to": null,
        "cumulativeGasUsed": "0x927c0",
        "gasUsed": "0x927c0",
        "logs": [],
        "logsBloom": "0x00000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000",
        "root": null,
        "transactionHash": "0x422fb0d5953c0c48cbb42fb58e1c30f5e150441c68374d70ca7d4f191fd56f26",
        "transactionIndex": "0x0",
        "status": "0x1",
        "effectiveGasPrice": "0x100"
        }"#;

    let _receipt: Receipt = serde_json::from_str(receipt_str).unwrap();
  }

  #[test]
  fn should_deserialize_receipt_without_gas() {
    let receipt_str = r#"{
        "blockHash": "0x83eaba432089a0bfe99e9fc9022d1cfcb78f95f407821be81737c84ae0b439c5",
        "blockNumber": "0x38",
        "contractAddress": "0x03d8c4566478a6e1bf75650248accce16a98509f",
        "from": "0x407d73d8a49eeb85d32cf465507dd71d507100c1",
        "to": "0x853f43d8a49eeb85d32cf465507dd71d507100c1",
        "cumulativeGasUsed": "0x927c0",
        "gasUsed": null,
        "logs": [],
        "logsBloom": "0x00000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000",
        "root": null,
        "transactionHash": "0x422fb0d5953c0c48cbb42fb58e1c30f5e150441c68374d70ca7d4f191fd56f26",
        "transactionIndex": "0x0",
        "status": "0x1",
        "effectiveGasPrice": "0x100"
    }"#;

    let _receipt: Receipt = serde_json::from_str(receipt_str).unwrap();
  }

  #[test]
  fn test_deserialize_signed_tx_parity() {
    // taken from RPC docs.
    let tx_str = r#"{
        "raw": "0xd46e8dd67c5d32be8d46e8dd67c5d32be8058bb8eb970870f072445675058bb8eb970870f072445675",
        "tx": {
          "hash": "0xc6ef2fc5426d6ad6fd9e2a26abeab0aa2411b7ab17f30a99d3cb96aed1d1055b",
          "nonce": "0x0",
          "blockHash": "0xbeab0aa2411b7ab17f30a99d3cb9c6ef2fc5426d6ad6fd9e2a26a6aed1d1055b",
          "blockNumber": "0x15df",
          "transactionIndex": "0x1",
          "from": "0x407d73d8a49eeb85d32cf465507dd71d507100c1",
          "to": "0x853f43d8a49eeb85d32cf465507dd71d507100c1",
          "value": "0x7f110",
          "gas": "0x7f110",
          "gasPrice": "0x09184e72a000",
          "input": "0x603880600c6000396000f300603880600c6000396000f3603880600c6000396000f360",
          "s": "0x777"
        }
    }"#;

    let _tx: RawTransaction = serde_json::from_str(tx_str).unwrap();
  }

  #[test]
  fn test_deserialize_signed_tx_geth() {
    let tx_str = r#"{
        "raw": "0xf85d01018094f3b3138e5eb1c75b43994d1bb760e2f9f735789680801ca06484d00575e961a7db35ebe5badaaca5cb7ee65d1f2f22f22da87c238b99d30da07a85d65797e4b555c1d3f64beebb2cb6f16a6fbd40c43cc48451eaf85305f66e",
        "tx": {
          "gas": "0x0",
          "gasPrice": "0x1",
          "hash": "0x0a32fb4e18bc6f7266a164579237b1b5c74271d453c04eab70444ca367d38418",
          "input": "0x",
          "nonce": "0x1",
          "to": "0xf3b3138e5eb1c75b43994d1bb760e2f9f7357896",
          "r": "0x6484d00575e961a7db35ebe5badaaca5cb7ee65d1f2f22f22da87c238b99d30d",
          "s": "0x7a85d65797e4b555c1d3f64beebb2cb6f16a6fbd40c43cc48451eaf85305f66e",
          "v": "0x1c",
          "value": "0x0"
        }
    }"#;

    let _tx: RawTransaction = serde_json::from_str(tx_str).unwrap();
  }
}
