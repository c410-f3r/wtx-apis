use crate::blockchain::solana::program::{LenBounds, PackData};
use wtx::collections::Vector;

create_data_enum! {
  #[derive(Debug, Eq, PartialEq)]
  pub enum Instruction {
    CreateIdempotent,
  }
}

impl PackData for Instruction {
  const LEN_BOUNDS: LenBounds = LenBounds::from_same(1);

  #[inline]
  fn len(&self) -> usize {
    match self {
      Instruction::CreateIdempotent => 1,
    }
  }

  #[inline]
  fn pack_data(&self, buffer: &mut Vector<u8>) -> crate::Result<()> {
    match *self {
      Self::CreateIdempotent => {
        1u8.pack_data(buffer)?;
      }
    }
    Ok(())
  }

  #[inline]
  fn unpack_data(bytes: &[u8]) -> crate::Result<Self> {
    match bytes.first() {
      Some(&1) => Ok(Self::CreateIdempotent),
      _ => Err(crate::Error::SolanaInvalidAccountData),
    }
  }
}
