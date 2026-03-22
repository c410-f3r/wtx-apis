use crate::blockchain::solana::{
  misc::sub_slice,
  program::{LenBounds, PackData},
};
use wtx::collection::Vector;

create_data_enum! {
  #[derive(Debug, Eq, PartialEq)]
  pub enum Instruction {
    SetComputeUnitLimit(u32),
    SetComputeUnitPrice(u64),
  }
}

impl PackData for Instruction {
  const LEN_BOUNDS: LenBounds = LenBounds::new(5, Some(9));

  fn len(&self) -> usize {
    let instance_len = match self {
      Instruction::SetComputeUnitLimit(el) => el.len(),
      Instruction::SetComputeUnitPrice(el) => el.len(),
    };
    1usize.saturating_add(instance_len)
  }

  fn pack_data(&self, buffer: &mut Vector<u8>) -> crate::Result<()> {
    match *self {
      Self::SetComputeUnitLimit(elem) => {
        2u8.pack_data(buffer)?;
        elem.pack_data(buffer)?;
      }
      Self::SetComputeUnitPrice(elem) => {
        3u8.pack_data(buffer)?;
        elem.pack_data(buffer)?;
      }
    }
    Ok(())
  }

  fn unpack_data(bytes: &[u8]) -> crate::Result<Self> {
    match bytes.first() {
      Some(&2) => Ok(Self::SetComputeUnitLimit(<_>::unpack_data(sub_slice(bytes, 1..5))?)),
      Some(&3) => Ok(Self::SetComputeUnitPrice(<_>::unpack_data(sub_slice(bytes, 1..9))?)),
      _ => Err(crate::Error::SolanaInvalidAccountData),
    }
  }
}
