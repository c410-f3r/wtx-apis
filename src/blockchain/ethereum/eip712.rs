use crate::blockchain::ethereum::{Address, SolTy, misc::keccak256, sol_int::SolInt};
use crypto_bigint::U256;
use wtx::collection::{ArrayVectorU8, ArrayWrapper, Vector};

/// EIP-712 domain
#[derive(Debug, serde::Serialize)]
pub struct Eip712Domain<'any> {
  /// Application's name
  #[serde(default, skip_serializing_if = "Option::is_none")]
  pub name: Option<&'any str>,
  /// The current major version
  #[serde(default, skip_serializing_if = "Option::is_none")]
  pub version: Option<&'any str>,
  /// EIP-155 chain ID
  #[serde(default, skip_serializing_if = "Option::is_none")]
  pub chain_id: Option<U256>,
  /// Verifying contract's address
  #[serde(default, skip_serializing_if = "Option::is_none")]
  pub verifying_contract: Option<Address>,
  /// Protocol's salt
  #[serde(default, skip_serializing_if = "Option::is_none")]
  pub salt: Option<[u8; 32]>,
}

impl<'any> Eip712Domain<'any> {
  pub(crate) const NAME: &'static str = "EIP712Domain";

  /// Shortcut
  pub fn new(
    name: Option<&'any str>,
    version: Option<&'any str>,
    chain_id: Option<U256>,
    verifying_contract: Option<Address>,
    salt: Option<[u8; 32]>,
  ) -> Self {
    Self { name, version, chain_id, verifying_contract, salt }
  }

  pub(crate) fn encode_data(&self, buffer: &mut Vector<u8>) -> crate::Result<()> {
    buffer.clear();
    let Self { name, version, chain_id, verifying_contract, salt } = self;
    if let Some(elem) = name {
      buffer.extend_from_copyable_slice(&keccak256([&elem.as_bytes().tokenize()?.0]))?;
    }
    if let Some(elem) = version {
      buffer.extend_from_copyable_slice(&keccak256([&elem.as_bytes().tokenize()?.0]))?;
    }
    if let Some(elem) = *chain_id {
      buffer.extend_from_copyable_slice(&SolInt(elem).tokenize()?.0)?;
    }
    if let Some(elem) = verifying_contract {
      buffer.extend_from_copyable_slice(&elem.tokenize()?.0)?;
    }
    if let Some(elem) = *salt {
      buffer.extend_from_copyable_slice(&ArrayWrapper(elem).tokenize()?.0)?;
    }
    Ok(())
  }

  pub(crate) fn encode_type(&self) -> ArrayVectorU8<u8, 98> {
    let Self { name, version, chain_id, verifying_contract, salt } = self;
    let mut buffer = ArrayVectorU8::new();
    let _rslt = buffer.extend_from_copyable_slices([Self::NAME.as_bytes(), &[b'(']]);
    if let Some(_) = name {
      let _rslt = buffer.extend_from_copyable_slice(b"string name,");
    }
    if let Some(_) = version {
      let _rslt = buffer.extend_from_copyable_slice(b"string version,");
    }
    if let Some(_) = chain_id {
      let _rslt = buffer.extend_from_copyable_slice(b"uint256 chainId,");
    }
    if let Some(_) = verifying_contract {
      let _rslt = buffer.extend_from_copyable_slice(b"address verifyingContract,");
    }
    if let Some(_) = salt {
      let _rslt = buffer.extend_from_copyable_slice(b"bytes32 salt");
    }
    if buffer.last().copied() == Some(b',') {
      let _ = buffer.pop();
    }
    let _rslt = buffer.push(b')');
    buffer
  }

  pub(crate) fn hash_struct(&self, buffer: &mut Vector<u8>) -> crate::Result<[u8; 32]> {
    self.encode_data(buffer)?;
    let rslt = keccak256([&self.type_hash()?, buffer]);
    Ok(rslt)
  }

  pub(crate) fn type_hash(&self) -> crate::Result<[u8; 32]> {
    Ok(keccak256([&self.encode_type()]))
  }
}

/// Auxiliary EIP712 methods
pub trait Eip712 {
  /// Domain
  fn domain(&self) -> Eip712Domain<'_>;

  /// Signing hash
  fn eip712_signing_hash(&self, buffer: &mut Vector<u8>) -> crate::Result<[u8; 32]> {
    let mut rslt = [0u8; 2 + 32 + 32];
    let mut vector = Vector::new();
    #[rustfmt::skip]
    let [
      a0, a1, a2, a3, a4, a5, a6, a7, a8, a9, a10, a11, a12, a13, a14, a15, a16, a17, a18,
      a19, a20, a21, a22, a23, a24, a25, a26, a27, a28, a29, a30, a31, a32, a33, a34, a35,
      a36, a37, a38, a39, a40, a41, a42, a43, a44, a45, a46, a47, a48, a49, a50, a51, a52,
      a53, a54, a55, a56, a57, a58, a59, a60, a61, a62, a63, a64, a65
    ] = &mut rslt;
    *a0 = 0b0001_1001;
    *a1 = 0b0000_0001;
    for (from, to) in self.domain().hash_struct(buffer)?.into_iter().zip([
      a2, a3, a4, a5, a6, a7, a8, a9, a10, a11, a12, a13, a14, a15, a16, a17, a18, a19, a20, a21,
      a22, a23, a24, a25, a26, a27, a28, a29, a30, a31, a32, a33,
    ]) {
      *to = from
    }
    for (from, to) in self.struct_hash(&mut vector)?.into_iter().zip([
      a34, a35, a36, a37, a38, a39, a40, a41, a42, a43, a44, a45, a46, a47, a48, a49, a50, a51,
      a52, a53, a54, a55, a56, a57, a58, a59, a60, a61, a62, a63, a64, a65,
    ]) {
      *to = from
    }
    Ok(keccak256([&rslt]))
  }

  /// Struct hash
  fn struct_hash(&self, buffer: &mut Vector<u8>) -> crate::Result<[u8; 32]>;
}
