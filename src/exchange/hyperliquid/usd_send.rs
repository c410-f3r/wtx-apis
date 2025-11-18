use crate::{
  blockchain::ethereum::{Eip712, Eip712Domain, SolInt, misc::keccak256},
  exchange::hyperliquid::{
    Chain,
    misc::{abi_encode_from_buffer, eip_712_domain},
  },
};
use alloc::string::ToString;
use rust_decimal::Decimal;
use wtx::collection::{ArrayWrapper, Vector};

#[derive(Debug, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct UsdSend<'any> {
  #[serde(serialize_with = "crate::exchange::hyperliquid::misc::serialize_hex")]
  pub(crate) signature_chain_id: u64,
  pub(crate) hyperliquid_chain: Chain,
  pub(crate) destination: &'any str,
  pub(crate) amount: Decimal,
  pub(crate) time: u64,
}

impl Eip712 for UsdSend<'_> {
  fn domain(&self) -> Eip712Domain<'_> {
    eip_712_domain(self.signature_chain_id)
  }

  fn struct_hash(&self, buffer: &mut Vector<u8>) -> crate::Result<[u8; 32]> {
    let items = (
      ArrayWrapper(keccak256(
        [b"HyperliquidTransaction:UsdSend(string hyperliquidChain,string destination,string amount,uint64 time)"]
      )),
      ArrayWrapper(keccak256([<&str>::from(self.hyperliquid_chain).as_bytes()])),
      ArrayWrapper(keccak256([self.destination.as_bytes()])),
      ArrayWrapper(keccak256([self.amount.to_string().as_bytes()])),
      SolInt(self.time),
    );
    abi_encode_from_buffer(buffer, &items)?;
    Ok(keccak256([buffer]))
  }
}
