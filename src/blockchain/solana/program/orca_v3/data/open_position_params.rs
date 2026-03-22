use crate::blockchain::solana::{
  misc::sub_slice,
  program::{LenBounds, PackData, TestingInstances},
};
use wtx::collection::Vector;

create_data_struct! {
  #[derive(Debug, Eq, PartialEq)]
  pub struct OpenPositionParams<> {
    pub bumps: u8,
    pub tick_lower_index: i32,
    pub tick_upper_index: i32,
  }
}

impl PackData for OpenPositionParams {
  const LEN_BOUNDS: LenBounds = LenBounds::from_same(9);

  fn pack_data(&self, buffer: &mut Vector<u8>) -> crate::Result<()> {
    let Self { bumps, tick_lower_index, tick_upper_index } = self;
    bumps.pack_data(buffer)?;
    tick_lower_index.pack_data(buffer)?;
    tick_upper_index.pack_data(buffer)?;
    Ok(())
  }

  fn unpack_data(bytes: &[u8]) -> crate::Result<Self> {
    Ok(Self {
      bumps: u8::unpack_data(sub_slice(bytes, 0..1))?,
      tick_lower_index: i32::unpack_data(sub_slice(bytes, 1..5))?,
      tick_upper_index: i32::unpack_data(sub_slice(bytes, 5..9))?,
    })
  }
}

impl TestingInstances for OpenPositionParams {
  fn min_instance() -> crate::Result<Self> {
    Ok(Self { bumps: 1, tick_lower_index: 2, tick_upper_index: 3 })
  }
}
