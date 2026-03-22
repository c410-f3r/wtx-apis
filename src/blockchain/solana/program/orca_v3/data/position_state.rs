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
  /// State of a liquidity position in a Whirlpool.
  pub struct Position<H> {
    /// The address of the Whirlpool this position belongs to.
    pub whirlpool: H,
    /// The address of the mint for the NFT representing this position.
    pub position_mint: H,
    /// The amount of liquidity held by this position.
    pub liquidity: u128,
    /// The lower tick boundary of the position.
    pub tick_lower_index: i32,
    /// The upper tick boundary of the position.
    pub tick_upper_index: i32,
    /// Fee growth checkpoint for token A.
    pub fee_growth_checkpoint_a: u128,
    /// Amount of fees owed to the position in token A.
    pub fee_owed_a: u64,
    /// Fee growth checkpoint for token B.
    pub fee_growth_checkpoint_b: u128,
    /// Amount of fees owed to the position in token B.
    pub fee_owed_b: u64,
    /// Information about rewards for this position.
    pub reward_infos: [u8; 72],
  }
}

impl PackData for Position<HashArray32Unit> {
  const LEN_BOUNDS: LenBounds = LenBounds::from_same(216);

  fn pack_data(&self, buffer: &mut Vector<u8>) -> crate::Result<()> {
    let Self {
      whirlpool,
      position_mint,
      liquidity,
      tick_lower_index,
      tick_upper_index,
      fee_growth_checkpoint_a,
      fee_owed_a,
      fee_growth_checkpoint_b,
      fee_owed_b,
      reward_infos,
    } = self;
    0u64.pack_data(buffer)?;
    buffer.extend_from_copyable_slice(whirlpool.bytes())?;
    buffer.extend_from_copyable_slice(position_mint.bytes())?;
    liquidity.pack_data(buffer)?;
    tick_lower_index.pack_data(buffer)?;
    tick_upper_index.pack_data(buffer)?;
    fee_growth_checkpoint_a.pack_data(buffer)?;
    fee_owed_a.pack_data(buffer)?;
    fee_growth_checkpoint_b.pack_data(buffer)?;
    fee_owed_b.pack_data(buffer)?;
    buffer.extend_from_copyable_slice(reward_infos)?;
    Ok(())
  }

  fn unpack_data(bytes: &[u8]) -> crate::Result<Self> {
    Ok(Self {
      whirlpool: HashArray32Unit::from_bytes(
        sub_slice(bytes, 8..40).try_into().map_err(wtx::Error::from)?,
      ),
      position_mint: HashArray32Unit::from_bytes(
        sub_slice(bytes, 40..72).try_into().map_err(wtx::Error::from)?,
      ),
      liquidity: <_>::unpack_data(sub_slice(bytes, 72..88))?,
      tick_lower_index: <_>::unpack_data(sub_slice(bytes, 88..92))?,
      tick_upper_index: <_>::unpack_data(sub_slice(bytes, 92..96))?,
      fee_growth_checkpoint_a: <_>::unpack_data(sub_slice(bytes, 96..112))?,
      fee_owed_a: <_>::unpack_data(sub_slice(bytes, 112..120))?,
      fee_growth_checkpoint_b: <_>::unpack_data(sub_slice(bytes, 120..136))?,
      fee_owed_b: <_>::unpack_data(sub_slice(bytes, 136..144))?,
      reward_infos: sub_slice(bytes, 144..216).try_into().map_err(wtx::Error::from)?,
    })
  }
}

impl TestingInstances for Position<HashArray32Unit> {
  fn min_instance() -> crate::Result<Self> {
    Ok(Self {
      whirlpool: HashArray32Unit::from_bytes([1; 32]),
      position_mint: HashArray32Unit::from_bytes([2; 32]),
      liquidity: 3,
      tick_lower_index: 4,
      tick_upper_index: 5,
      fee_growth_checkpoint_a: 6,
      fee_owed_a: 7,
      fee_growth_checkpoint_b: 8,
      fee_owed_b: 9,
      reward_infos: [10; 72],
    })
  }
}

impl TryFrom<Position<HashArray32Unit>> for Position<HashArray32ArrayString44> {
  type Error = crate::Error;

  fn try_from(from: Position<HashArray32Unit>) -> Result<Self, Self::Error> {
    Ok(Self {
      whirlpool: HashArray32ArrayString44::from_base58_bytes(from.whirlpool.into_bytes())?,
      position_mint: HashArray32ArrayString44::from_base58_bytes(from.position_mint.into_bytes())?,
      liquidity: from.liquidity,
      tick_lower_index: from.tick_lower_index,
      tick_upper_index: from.tick_upper_index,
      fee_growth_checkpoint_a: from.fee_growth_checkpoint_a,
      fee_owed_a: from.fee_owed_a,
      fee_growth_checkpoint_b: from.fee_growth_checkpoint_b,
      fee_owed_b: from.fee_owed_b,
      reward_infos: from.reward_infos,
    })
  }
}
