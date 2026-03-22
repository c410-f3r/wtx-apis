use crate::{
  blockchain::solana::{
    misc::sub_slice,
    program::{LenBounds, PackData, TestingInstances},
  },
  misc::HashArray32Unit,
};
use wtx::{
  collection::{ArrayStringU8, Vector},
  misc::{Usize, from_utf8_basic},
};

create_data_struct! {
  #[derive(Debug, Eq, PartialEq)]
  pub struct CreateAccountWithSeedParams<H> {
    pub base: H,
    pub seed: ArrayStringU8<32>,
    pub lamports: u64,
    pub space: u64,
    pub owner: H,
  }
}

impl PackData for CreateAccountWithSeedParams<HashArray32Unit> {
  const LEN_BOUNDS: LenBounds = LenBounds::new(88, Some(96));

  fn len(&self) -> usize {
    88usize.wrapping_add(*Usize::from(self.seed.len()))
  }

  fn pack_data(&self, buffer: &mut Vector<u8>) -> crate::Result<()> {
    let Self { base, seed, lamports, space, owner } = self;
    buffer.extend_from_copyable_slice(base.bytes())?;
    {
      u64::from(seed.len()).pack_data(buffer)?;
      buffer.extend_from_copyable_slice(seed.as_str().as_bytes())?;
    };
    lamports.pack_data(buffer)?;
    space.pack_data(buffer)?;
    buffer.extend_from_copyable_slice(owner.bytes())?;
    Ok(())
  }

  fn unpack_data(bytes: &[u8]) -> crate::Result<Self> {
    let base =
      HashArray32Unit::from_bytes(sub_slice(bytes, 0..32).try_into().map_err(wtx::Error::from)?);
    let mut range;
    let seed = {
      let len =
        usize::try_from(u64::unpack_data(sub_slice(bytes, 32..40))?).map_err(wtx::Error::from)?;
      range = [40, 40usize.saturating_add(len)];
      ArrayStringU8::try_from(
        from_utf8_basic(sub_slice(bytes, range[0]..range[1])).map_err(wtx::Error::from)?,
      )?
    };
    range = [range[1], range[1].saturating_add(8)];
    let lamports = u64::unpack_data(sub_slice(bytes, range[0]..range[1]))?;
    range = [range[1], range[1].saturating_add(8)];
    let space = u64::unpack_data(sub_slice(bytes, range[0]..range[1]))?;
    range = [range[1], range[1].saturating_add(32)];
    let owner = HashArray32Unit::from_bytes(
      sub_slice(bytes, range[0]..range[1]).try_into().map_err(wtx::Error::from)?,
    );
    Ok(Self { base, seed, lamports, space, owner })
  }
}

impl TestingInstances for CreateAccountWithSeedParams<HashArray32Unit> {
  fn min_instance() -> crate::Result<Self> {
    Ok(Self {
      base: HashArray32Unit::from_bytes([1; 32]),
      seed: <_>::default(),
      lamports: 3,
      space: 4,
      owner: HashArray32Unit::from_bytes([5; 32]),
    })
  }

  fn variable_instance() -> crate::Result<Self> {
    Ok(Self {
      base: HashArray32Unit::from_bytes([1; 32]),
      seed: ArrayStringU8::try_from("2345")?,
      lamports: 3,
      space: 4,
      owner: HashArray32Unit::from_bytes([5; 32]),
    })
  }
}
