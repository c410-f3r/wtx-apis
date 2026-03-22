use crate::blockchain::solana::{
  misc::sub_slice,
  program::{LenBounds, PackData, TestingInstances},
};
use wtx::collection::Vector;

create_data_struct! {
  #[derive(Debug, Eq, PartialEq)]
  pub struct IncreaseLiquidityParams<> {
    pub liquidity_amount: u128,
    pub token_max_a: u64,
    pub token_max_b: u64,
  }
}

impl PackData for IncreaseLiquidityParams {
  const LEN_BOUNDS: LenBounds = LenBounds::from_same(32);

  fn pack_data(&self, buffer: &mut Vector<u8>) -> crate::Result<()> {
    let Self { liquidity_amount, token_max_a, token_max_b } = self;
    liquidity_amount.pack_data(buffer)?;
    token_max_a.pack_data(buffer)?;
    token_max_b.pack_data(buffer)?;
    Ok(())
  }

  fn unpack_data(bytes: &[u8]) -> crate::Result<Self> {
    Ok(Self {
      liquidity_amount: u128::unpack_data(sub_slice(bytes, 0..16))?,
      token_max_a: u64::unpack_data(sub_slice(bytes, 16..24))?,
      token_max_b: u64::unpack_data(sub_slice(bytes, 24..32))?,
    })
  }
}

impl TestingInstances for IncreaseLiquidityParams {
  fn min_instance() -> crate::Result<Self> {
    Ok(Self { liquidity_amount: 1, token_max_a: 2, token_max_b: 3 })
  }
}
