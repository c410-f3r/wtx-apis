use crate::blockchain::ethereum::{
  Address, Eip712, Eip712Domain, PackedSeq, SolStruct, SolTy, Word,
};
use k256::U256;
use wtx::collection::{ArrayWrapper, Vector};

type AgentTuple = (&'static [u8], ArrayWrapper<u8, 32>);

#[derive(Debug)]
pub struct Agent<'source> {
  pub source: &'source [u8],
  pub connection_id: ArrayWrapper<u8, 32>,
}

impl Eip712 for Agent<'_> {
  fn domain(&self) -> Eip712Domain<'_> {
    Eip712Domain::new(
      Some("Exchange"),
      Some("1"),
      Some(U256::from_u16(1337)),
      Some(Address([0; 20])),
      None,
    )
  }

  fn struct_hash(&self, buffer: &mut Vector<u8>) -> crate::Result<[u8; 32]> {
    self.eip712_hash_struct(buffer)
  }
}

impl<'de> SolStruct<'de> for Agent<'de> {
  fn eip712_components() -> &'static [&'static str] {
    &[]
  }

  fn eip712_encode_data(&self, buffer: &mut Vector<u8>) -> crate::Result<impl AsRef<[u8]>> {
    let mut rslt = [0u8; 64];
    let lhs = self.source.eip712_data_word(buffer)?.0;
    let rhs = self.connection_id.eip712_data_word(buffer)?.0;
    rslt[..32].copy_from_slice(&lhs);
    rslt[32..].copy_from_slice(&rhs);
    Ok(rslt)
  }

  fn eip712_encode_type() -> &'static str {
    Self::eip712_root_type()
  }

  fn eip712_root_type() -> &'static str {
    "Agent(string source,bytes32 connectionId)"
  }
}

impl<'de> SolTy<'de> for Agent<'de> {
  const ENCODED_SIZE: Option<usize> = None;
  const PACKED_ENCODED_SIZE: Option<usize> = None;
  const SOL_NAME: &'static str = "Agent";

  type DeToken<'any> = Self;
  type Token<'any> = (PackedSeq<'any>, Word);

  fn abi_encode_packed(&self, buffer: &mut Vector<u8>) -> crate::Result<()> {
    (self.source, self.connection_id).abi_encode_packed(buffer)
  }

  fn abi_encoded_size(&self) -> usize {
    if let Some(size) = Self::ENCODED_SIZE {
      return size;
    }
    (self.source, self.connection_id).abi_encoded_size()
  }

  fn abi_packed_encoded_size(&self) -> usize {
    if let Some(size) = Self::PACKED_ENCODED_SIZE {
      return size;
    }
    (self.source, self.connection_id).abi_packed_encoded_size()
  }

  fn detokenize(token: Self::Token<'de>) -> crate::Result<Self::DeToken<'de>> {
    Ok(Self { source: token.0.0, connection_id: ArrayWrapper(token.1.0) })
  }

  fn eip712_data_word(&self, buffer: &mut Vector<u8>) -> crate::Result<Word> {
    Ok(Word(self.eip712_hash_struct(buffer)?))
  }

  fn tokenize(&self) -> crate::Result<Self::Token<'_>> {
    Ok((self.source.tokenize()?, self.connection_id.tokenize()?))
  }

  fn valid_token(token: &Self::Token<'_>) -> bool {
    AgentTuple::valid_token(token)
  }
}

#[cfg(test)]
mod tests {
  use crate::{
    blockchain::ethereum::{PackedSeq, SolTy, Word},
    exchange::hyperliquid::agent::Agent,
  };
  use wtx::{
    collection::Vector,
    de::{HexDecMode, decode_hex_to_slice},
  };

  #[test]
  fn basic() {
    let agent = Agent {
      source: b"abc",
      connection_id: [
        1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25,
        26, 27, 28, 29, 30, 31, 32,
      ]
      .into(),
    };

    assert_eq!(
      &agent.eip712_data_word(&mut Vector::new()).unwrap().0,
      decode_hex_to_slice(
        b"96e29d9f3d1099610ffecd9159335362360a93f5d2e4d3147c8a76710d16b51d",
        HexDecMode::Automatic,
        &mut [0; 32]
      )
      .unwrap()
    );

    let mut word = [0; 32];
    decode_hex_to_slice(
      b"0102030405060708090a0b0c0d0e0f101112131415161718191a1b1c1d1e1f20",
      HexDecMode::Automatic,
      &mut word,
    )
    .unwrap();
    assert_eq!(
      agent.tokenize().unwrap(),
      (
        PackedSeq(decode_hex_to_slice(b"616263", HexDecMode::Automatic, &mut [0; 3]).unwrap()),
        Word(word)
      )
    );
  }
}
