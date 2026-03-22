use crate::{
  blockchain::solana::{
    misc::sub_slice,
    program::{LenBounds, PackData, TestingInstances},
  },
  misc::HashArray32Unit,
};
use wtx::collection::Vector;

create_data_struct! {
  #[derive(Debug, Eq, PartialEq)]
  pub struct CreateAccountParams<H> {
    pub lamports: u64,
    pub space: u64,
    pub owner: H,
  }
}

impl PackData for CreateAccountParams<HashArray32Unit> {
  const LEN_BOUNDS: LenBounds = LenBounds::from_same(48);

  fn pack_data(&self, buffer: &mut Vector<u8>) -> crate::Result<()> {
    let Self { lamports, space, owner } = self;
    lamports.pack_data(buffer)?;
    space.pack_data(buffer)?;
    buffer.extend_from_copyable_slice(owner.bytes())?;
    Ok(())
  }

  fn unpack_data(bytes: &[u8]) -> crate::Result<Self> {
    let lamports = u64::unpack_data(sub_slice(bytes, 0..8))?;
    let space = u64::unpack_data(sub_slice(bytes, 8..16))?;
    let owner = HashArray32Unit::from_bytes(
      sub_slice(bytes, 16..Self::LEN_BOUNDS.max_or_min()).try_into().map_err(wtx::Error::from)?,
    );
    Ok(Self { lamports, space, owner })
  }
}

impl TestingInstances for CreateAccountParams<HashArray32Unit> {
  fn min_instance() -> crate::Result<Self> {
    Ok(Self { lamports: 1, space: 2, owner: HashArray32Unit::from_bytes([3; 32]) })
  }
}
