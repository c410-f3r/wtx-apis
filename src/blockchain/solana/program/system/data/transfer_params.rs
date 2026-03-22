use crate::blockchain::solana::program::{LenBounds, PackData, TestingInstances};
use wtx::collection::Vector;

create_data_struct! {
  #[derive(Debug, Eq, PartialEq)]
  pub struct TransferParams<> {
    pub lamports: u64,
  }
}

impl PackData for TransferParams {
  const LEN_BOUNDS: LenBounds = LenBounds::from_same(8);

  fn pack_data(&self, buffer: &mut Vector<u8>) -> crate::Result<()> {
    let Self { lamports } = self;
    lamports.pack_data(buffer)?;
    Ok(())
  }

  fn unpack_data(bytes: &[u8]) -> crate::Result<Self> {
    Ok(Self { lamports: <_>::unpack_data(bytes)? })
  }
}

impl TestingInstances for TransferParams {
  fn min_instance() -> crate::Result<Self> {
    Ok(Self { lamports: 1 })
  }
}
