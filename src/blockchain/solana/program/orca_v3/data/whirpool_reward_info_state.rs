use crate::{
  blockchain::solana::{
    misc::sub_slice,
    program::{LenBounds, PackData, TestingInstances},
  },
  misc::{HashArray32ArrayString44, HashArray32Unit},
};
use wtx::collection::Vector;

create_data_struct! {
  #[derive(Debug, PartialEq)]
  pub struct WhirlpoolRewardInfo<H> {
    pub mint: H,
    pub vault: H,
    pub authority: H,
    pub emissions_per_second_x64: u128,
    pub growth_global_x64: u128,
  }
}

impl PackData for WhirlpoolRewardInfo<HashArray32Unit> {
  const LEN_BOUNDS: LenBounds = LenBounds::from_same(128);

  fn pack_data(&self, buffer: &mut Vector<u8>) -> crate::Result<()> {
    let Self { mint, vault, authority, emissions_per_second_x64, growth_global_x64 } = self;
    buffer.extend_from_copyable_slice(mint.bytes())?;
    buffer.extend_from_copyable_slice(vault.bytes())?;
    buffer.extend_from_copyable_slice(authority.bytes())?;
    emissions_per_second_x64.pack_data(buffer)?;
    growth_global_x64.pack_data(buffer)?;
    Ok(())
  }

  fn unpack_data(bytes: &[u8]) -> crate::Result<Self> {
    Ok(Self {
      mint: HashArray32Unit::from_bytes(
        sub_slice(bytes, 0..32).try_into().map_err(wtx::Error::from)?,
      ),
      vault: HashArray32Unit::from_bytes(
        sub_slice(bytes, 32..64).try_into().map_err(wtx::Error::from)?,
      ),
      authority: HashArray32Unit::from_bytes(
        sub_slice(bytes, 64..96).try_into().map_err(wtx::Error::from)?,
      ),
      emissions_per_second_x64: <_>::unpack_data(sub_slice(bytes, 96..112))?,
      growth_global_x64: <_>::unpack_data(sub_slice(bytes, 112..128))?,
    })
  }
}

impl TestingInstances for WhirlpoolRewardInfo<HashArray32Unit> {
  fn min_instance() -> crate::Result<Self> {
    Ok(Self {
      mint: HashArray32Unit::from_bytes([1; 32]),
      vault: HashArray32Unit::from_bytes([2; 32]),
      authority: HashArray32Unit::from_bytes([3; 32]),
      emissions_per_second_x64: 4,
      growth_global_x64: 5,
    })
  }
}

impl TryFrom<WhirlpoolRewardInfo<HashArray32Unit>>
  for WhirlpoolRewardInfo<HashArray32ArrayString44>
{
  type Error = crate::Error;

  fn try_from(from: WhirlpoolRewardInfo<HashArray32Unit>) -> Result<Self, Self::Error> {
    Ok(Self {
      mint: HashArray32ArrayString44::from_base58_bytes(from.mint.into_bytes())?,
      vault: HashArray32ArrayString44::from_base58_bytes(from.vault.into_bytes())?,
      authority: HashArray32ArrayString44::from_base58_bytes(from.authority.into_bytes())?,
      emissions_per_second_x64: from.emissions_per_second_x64,
      growth_global_x64: from.growth_global_x64,
    })
  }
}
