use crate::blockchain::solana::{
  misc::sub_slice,
  program::{LenBounds, PackData, TestingInstances},
};
use wtx::collection::Vector;

create_data_struct! {
  #[derive(Debug, Eq, PartialEq)]
  pub struct TransferCheckedParams<> {
    pub amount: u64,
    pub decimals: u8,
  }
}

impl PackData for TransferCheckedParams {
  const LEN_BOUNDS: LenBounds = LenBounds::from_same(9);

  fn pack_data(&self, buffer: &mut Vector<u8>) -> crate::Result<()> {
    let Self { amount, decimals } = self;
    amount.pack_data(buffer)?;
    decimals.pack_data(buffer)?;
    Ok(())
  }

  fn unpack_data(bytes: &[u8]) -> crate::Result<Self> {
    Ok(Self {
      amount: u64::unpack_data(sub_slice(bytes, 0..8))?,
      decimals: u8::unpack_data(sub_slice(bytes, 8..9))?,
    })
  }
}

impl TestingInstances for TransferCheckedParams {
  fn min_instance() -> crate::Result<Self> {
    Ok(Self { amount: 1, decimals: 2 })
  }
}
