use crate::blockchain::solana::{
  misc::sub_slice,
  program::{LenBounds, PackData, TestingInstances},
};
use wtx::collection::Vector;

create_data_struct! {
  #[derive(Debug, Eq, PartialEq)]
  pub struct TransferParams<> {
    pub amount: u64,
  }
}

impl PackData for TransferParams {
  const LEN_BOUNDS: LenBounds = LenBounds::from_same(8);

  fn pack_data(&self, buffer: &mut Vector<u8>) -> crate::Result<()> {
    let Self { amount } = self;
    amount.pack_data(buffer)?;
    Ok(())
  }

  fn unpack_data(bytes: &[u8]) -> crate::Result<Self> {
    Ok(Self { amount: u64::unpack_data(sub_slice(bytes, 0..8))? })
  }
}

impl TestingInstances for TransferParams {
  fn min_instance() -> crate::Result<Self> {
    Ok(Self { amount: 1 })
  }
}
