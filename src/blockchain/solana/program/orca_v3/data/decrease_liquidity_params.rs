use crate::blockchain::solana::{
  misc::sub_slice,
  program::{LenBounds, PackData, TestingInstances},
};
use wtx::collection::Vector;
create_data_struct! {
  #[derive(Debug, Eq, PartialEq)]
  pub struct DecreaseLiquidityParams<> {
    pub liquidity_amount: u128,
    pub token_min_a: u64,
    pub token_min_b: u64,
  }
}

impl PackData for DecreaseLiquidityParams {
  const LEN_BOUNDS: LenBounds = LenBounds::from_same(32);

  fn pack_data(&self, buffer: &mut Vector<u8>) -> crate::Result<()> {
    let Self { liquidity_amount, token_min_a, token_min_b } = self;
    liquidity_amount.pack_data(buffer)?;
    token_min_a.pack_data(buffer)?;
    token_min_b.pack_data(buffer)?;
    Ok(())
  }

  fn unpack_data(bytes: &[u8]) -> crate::Result<Self> {
    Ok(Self {
      liquidity_amount: u128::unpack_data(sub_slice(bytes, 0..16))?,
      token_min_a: u64::unpack_data(sub_slice(bytes, 16..24))?,
      token_min_b: u64::unpack_data(sub_slice(bytes, 24..32))?,
    })
  }
}

impl TestingInstances for DecreaseLiquidityParams {
  fn min_instance() -> crate::Result<Self> {
    Ok(Self { liquidity_amount: 1, token_min_a: 2, token_min_b: 3 })
  }
}
