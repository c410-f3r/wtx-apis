use crate::{
  blockchain::ethereum::{
    Eip712, Eip712Domain,
    misc::{abi_encode_from_buffer, keccak256},
  },
  exchange::aster::EIP712_DOMAIN,
};
use wtx::collection::{ArrayWrapper, Vector};

/// Used to encode a decentralized message
#[derive(Debug)]
pub(crate) struct Message<'any> {
  /// Set of grouped order parameters
  pub(crate) msg: &'any [u8],
}

impl Eip712 for Message<'_> {
  #[inline]
  fn domain(&self) -> Eip712Domain<'_> {
    EIP712_DOMAIN
  }

  #[inline]
  fn struct_hash(&self, buffer: &mut Vector<u8>) -> crate::Result<[u8; 32]> {
    let items =
      (ArrayWrapper(keccak256([b"Message(string msg)"])), ArrayWrapper(keccak256([self.msg])));
    abi_encode_from_buffer(buffer, &items)?;
    Ok(keccak256([buffer]))
  }
}
