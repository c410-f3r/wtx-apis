use crate::{
  blockchain::solana::{
    misc::sub_slice,
    program::{LenBounds, PackData, TestingInstances, orca_v3::data::WhirlpoolRewardInfo},
  },
  misc::{HashArray32ArrayString44, HashArray32Unit},
};
use wtx::collection::Vector;

create_data_struct! {
  #[derive(Debug, PartialEq)]
  pub struct Whirlpool<H> {
    pub whirlpools_config: H,
    pub whirlpool_bump: [u8; 1],
    pub tick_spacing: u16,
    pub tick_spacing_seed: [u8; 2],
    pub fee_rate: u16,
    pub protocol_fee_rate: u16,
    pub liquidity: u128,
    pub sqrt_price: u128,
    pub tick_current_index: i32,
    pub protocol_fee_owed_a: u64,
    pub protocol_fee_owed_b: u64,
    pub token_mint_a: H,
    pub token_vault_a: H,
    pub fee_growth_global_a: u128,
    pub token_mint_b: H,
    pub token_vault_b: H,
    pub fee_growth_global_b: u128,
    pub reward_last_updated_timestamp: u64,
    pub reward_infos: [WhirlpoolRewardInfo<H>; 3],
  }
}

impl PackData for Whirlpool<HashArray32Unit> {
  const LEN_BOUNDS: LenBounds = LenBounds::from_same(653);

  fn pack_data(&self, buffer: &mut Vector<u8>) -> crate::Result<()> {
    let Self {
      whirlpools_config,
      whirlpool_bump,
      tick_spacing,
      tick_spacing_seed,
      fee_rate,
      protocol_fee_rate,
      liquidity,
      sqrt_price,
      tick_current_index,
      protocol_fee_owed_a,
      protocol_fee_owed_b,
      token_mint_a,
      token_vault_a,
      fee_growth_global_a,
      token_mint_b,
      token_vault_b,
      fee_growth_global_b,
      reward_last_updated_timestamp,
      reward_infos: [a, b, c],
    } = self;
    0u64.pack_data(buffer)?;
    buffer.extend_from_copyable_slice(whirlpools_config.bytes())?;
    buffer.extend_from_copyable_slice(whirlpool_bump)?;
    tick_spacing.pack_data(buffer)?;
    buffer.extend_from_copyable_slice(tick_spacing_seed)?;
    fee_rate.pack_data(buffer)?;
    protocol_fee_rate.pack_data(buffer)?;
    liquidity.pack_data(buffer)?;
    sqrt_price.pack_data(buffer)?;
    tick_current_index.pack_data(buffer)?;
    protocol_fee_owed_a.pack_data(buffer)?;
    protocol_fee_owed_b.pack_data(buffer)?;
    buffer.extend_from_copyable_slice(token_mint_a.bytes())?;
    buffer.extend_from_copyable_slice(token_vault_a.bytes())?;
    fee_growth_global_a.pack_data(buffer)?;
    buffer.extend_from_copyable_slice(token_mint_b.bytes())?;
    buffer.extend_from_copyable_slice(token_vault_b.bytes())?;
    fee_growth_global_b.pack_data(buffer)?;
    reward_last_updated_timestamp.pack_data(buffer)?;
    a.pack_data(buffer)?;
    b.pack_data(buffer)?;
    c.pack_data(buffer)?;
    Ok(())
  }

  fn unpack_data(bytes: &[u8]) -> crate::Result<Self> {
    Ok(Self {
      whirlpools_config: HashArray32Unit::from_bytes(
        sub_slice(bytes, 8..40).try_into().map_err(wtx::Error::from)?,
      ),
      whirlpool_bump: sub_slice(bytes, 40..41).try_into().map_err(wtx::Error::from)?,
      tick_spacing: <_>::unpack_data(sub_slice(bytes, 41..43))?,
      tick_spacing_seed: sub_slice(bytes, 43..45).try_into().map_err(wtx::Error::from)?,
      fee_rate: <_>::unpack_data(sub_slice(bytes, 45..47))?,
      protocol_fee_rate: <_>::unpack_data(sub_slice(bytes, 47..49))?,
      liquidity: <_>::unpack_data(sub_slice(bytes, 49..65))?,
      sqrt_price: <_>::unpack_data(sub_slice(bytes, 65..81))?,
      tick_current_index: <_>::unpack_data(sub_slice(bytes, 81..85))?,
      protocol_fee_owed_a: <_>::unpack_data(sub_slice(bytes, 85..93))?,
      protocol_fee_owed_b: <_>::unpack_data(sub_slice(bytes, 93..101))?, //
      token_mint_a: HashArray32Unit::from_bytes(
        sub_slice(bytes, 101..133).try_into().map_err(wtx::Error::from)?,
      ),
      token_vault_a: HashArray32Unit::from_bytes(
        sub_slice(bytes, 133..165).try_into().map_err(wtx::Error::from)?,
      ),
      fee_growth_global_a: <_>::unpack_data(sub_slice(bytes, 165..181))?,
      token_mint_b: HashArray32Unit::from_bytes(
        sub_slice(bytes, 181..213).try_into().map_err(wtx::Error::from)?,
      ),
      token_vault_b: HashArray32Unit::from_bytes(
        sub_slice(bytes, 213..245).try_into().map_err(wtx::Error::from)?,
      ),
      fee_growth_global_b: <_>::unpack_data(sub_slice(bytes, 245..261))?,
      reward_last_updated_timestamp: <_>::unpack_data(sub_slice(bytes, 261..269))?,
      reward_infos: [
        <_>::unpack_data(sub_slice(bytes, 269..397))?,
        <_>::unpack_data(sub_slice(bytes, 397..525))?,
        <_>::unpack_data(sub_slice(bytes, 525..653))?,
      ],
    })
  }
}

impl TestingInstances for Whirlpool<HashArray32Unit> {
  fn min_instance() -> crate::Result<Self> {
    Ok(Self {
      whirlpools_config: HashArray32Unit::from_bytes([1; 32]),
      whirlpool_bump: [2],
      tick_spacing: 2,
      tick_spacing_seed: [3; 2],
      fee_rate: 4,
      protocol_fee_rate: 5,
      liquidity: 6,
      sqrt_price: 7,
      tick_current_index: 8,
      protocol_fee_owed_a: 9,
      protocol_fee_owed_b: 10,
      token_mint_a: HashArray32Unit::from_bytes([11; 32]),
      token_vault_a: HashArray32Unit::from_bytes([12; 32]),
      fee_growth_global_a: 13,
      token_mint_b: HashArray32Unit::from_bytes([14; 32]),
      token_vault_b: HashArray32Unit::from_bytes([15; 32]),
      fee_growth_global_b: 16,
      reward_last_updated_timestamp: 17,
      reward_infos: [
        WhirlpoolRewardInfo::min_instance()?,
        WhirlpoolRewardInfo::min_instance()?,
        WhirlpoolRewardInfo::min_instance()?,
      ],
    })
  }
}

impl TryFrom<Whirlpool<HashArray32Unit>> for Whirlpool<HashArray32ArrayString44> {
  type Error = crate::Error;

  fn try_from(from: Whirlpool<HashArray32Unit>) -> Result<Self, Self::Error> {
    Ok(Self {
      whirlpools_config: HashArray32ArrayString44::from_base58_bytes(
        from.whirlpools_config.into_bytes(),
      )?,
      whirlpool_bump: from.whirlpool_bump,
      tick_spacing: from.tick_spacing,
      tick_spacing_seed: from.tick_spacing_seed,
      fee_rate: from.fee_rate,
      protocol_fee_rate: from.protocol_fee_rate,
      liquidity: from.liquidity,
      sqrt_price: from.sqrt_price,
      tick_current_index: from.tick_current_index,
      protocol_fee_owed_a: from.protocol_fee_owed_a,
      protocol_fee_owed_b: from.protocol_fee_owed_b,
      token_mint_a: HashArray32ArrayString44::from_base58_bytes(from.token_mint_a.into_bytes())?,
      token_vault_a: HashArray32ArrayString44::from_base58_bytes(from.token_vault_a.into_bytes())?,
      fee_growth_global_a: from.fee_growth_global_a,
      token_mint_b: HashArray32ArrayString44::from_base58_bytes(from.token_mint_b.into_bytes())?,
      token_vault_b: HashArray32ArrayString44::from_base58_bytes(from.token_vault_b.into_bytes())?,
      fee_growth_global_b: from.fee_growth_global_b,
      reward_last_updated_timestamp: from.reward_last_updated_timestamp,
      reward_infos: {
        let [a, b, c] = from.reward_infos;
        [
          WhirlpoolRewardInfo::try_from(a)?,
          WhirlpoolRewardInfo::try_from(b)?,
          WhirlpoolRewardInfo::try_from(c)?,
        ]
      },
    })
  }
}
