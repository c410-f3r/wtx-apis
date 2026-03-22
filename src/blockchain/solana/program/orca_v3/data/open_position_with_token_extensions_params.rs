use crate::blockchain::solana::{
  misc::sub_slice,
  program::{LenBounds, PackData, TestingInstances},
};
use wtx::collection::Vector;

create_data_struct! {
  #[derive(Debug, Eq, PartialEq)]
  pub struct OpenPositionWithTokenExtensionsParams<> {
    pub tick_lower_index: i32,
    pub tick_upper_index: i32,
    pub with_token_metadata: bool,
  }
}

impl PackData for OpenPositionWithTokenExtensionsParams {
  const LEN_BOUNDS: LenBounds = LenBounds::from_same(9);

  fn pack_data(&self, buffer: &mut Vector<u8>) -> crate::Result<()> {
    let Self { tick_lower_index, tick_upper_index, with_token_metadata } = self;
    tick_lower_index.pack_data(buffer)?;
    tick_upper_index.pack_data(buffer)?;
    with_token_metadata.pack_data(buffer)?;
    Ok(())
  }

  fn unpack_data(bytes: &[u8]) -> crate::Result<Self> {
    Ok(Self {
      tick_lower_index: i32::unpack_data(sub_slice(bytes, 0..4))?,
      tick_upper_index: i32::unpack_data(sub_slice(bytes, 4..8))?,
      with_token_metadata: bool::unpack_data(sub_slice(bytes, 8..9))?,
    })
  }
}

impl TestingInstances for OpenPositionWithTokenExtensionsParams {
  fn min_instance() -> crate::Result<Self> {
    Ok(Self { tick_lower_index: 1, tick_upper_index: 2, with_token_metadata: true })
  }
}
