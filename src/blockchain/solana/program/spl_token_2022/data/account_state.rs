use crate::{
  blockchain::solana::{
    misc::{
      pack_coption_address, pack_coption_u64, sub_slice, unpack_coption_address, unpack_coption_u64,
    },
    program::{LenBounds, PackData, TestingInstances, spl_token_2022::data::AccountStatus},
  },
  misc::{Hash, HashArray32Unit},
};
use wtx::collection::Vector;

create_data_struct! {
  #[derive(Debug, Eq, PartialEq)]
  #[repr(C)]
  pub struct AccountState<H> {
    pub mint: H,
    pub owner: H,
    pub amount: u64,
    pub delegate: Option<H>,
    pub state: AccountStatus,
    pub is_native: Option<u64>,
    pub delegated_amount: u64,
    pub close_authority: Option<H>,
  }
}

impl PackData for AccountState<HashArray32Unit> {
  const LEN_BOUNDS: LenBounds = LenBounds::from_same(165);

  fn pack_data(&self, buffer: &mut Vector<u8>) -> crate::Result<()> {
    let AccountState {
      mint,
      owner,
      amount,
      delegate,
      state,
      is_native,
      delegated_amount,
      close_authority,
    } = self;
    buffer.extend_from_copyable_slice(mint.bytes())?;
    buffer.extend_from_copyable_slice(owner.bytes())?;
    amount.pack_data(buffer)?;
    pack_coption_address(delegate.as_ref().map(Hash::bytes), buffer)?;
    state.pack_data(buffer)?;
    pack_coption_u64(is_native.as_ref(), buffer)?;
    delegated_amount.pack_data(buffer)?;
    pack_coption_address(close_authority.as_ref().map(Hash::bytes), buffer)?;
    Ok(())
  }

  fn unpack_data(bytes: &[u8]) -> crate::Result<Self> {
    Ok(AccountState {
      mint: HashArray32Unit::from_bytes(
        sub_slice(bytes, 0..32).try_into().map_err(wtx::Error::from)?,
      ),
      owner: HashArray32Unit::from_bytes(
        sub_slice(bytes, 32..64).try_into().map_err(wtx::Error::from)?,
      ),
      amount: u64::unpack_data(sub_slice(bytes, 64..72))?,
      delegate: unpack_coption_address(sub_slice(bytes, 72..108))?.map(HashArray32Unit::from_bytes),
      state: <_>::unpack_data(sub_slice(bytes, 108..109))?,
      is_native: unpack_coption_u64(sub_slice(bytes, 109..121))?,
      delegated_amount: u64::unpack_data(sub_slice(bytes, 121..129))?,
      close_authority: unpack_coption_address(sub_slice(
        bytes,
        129..Self::LEN_BOUNDS.max_or_min(),
      ))?
      .map(HashArray32Unit::from_bytes),
    })
  }
}

impl TestingInstances for AccountState<HashArray32Unit> {
  fn min_instance() -> crate::Result<Self> {
    Ok(Self {
      mint: HashArray32Unit::from_bytes([1; 32]),
      owner: HashArray32Unit::from_bytes([2; 32]),
      amount: 3,
      delegate: Some(HashArray32Unit::from_bytes([4; 32])),
      state: AccountStatus::Initialized,
      is_native: Some(6),
      delegated_amount: 8,
      close_authority: Some(HashArray32Unit::from_bytes([8; 32])),
    })
  }
}
