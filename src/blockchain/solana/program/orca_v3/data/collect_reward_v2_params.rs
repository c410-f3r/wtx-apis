use crate::blockchain::solana::{
  misc::sub_slice,
  program::{
    LenBounds, PackData, TestingInstances,
    orca_v3::data::{AccountsType, RemainingAccountsInfo, RemainingAccountsSlice},
  },
};
use wtx::collection::Vector;

create_data_struct! {
  #[derive(Debug, Eq, PartialEq)]
  pub struct CollectRewardV2Params<> {
    pub reward_index: u8,
    pub remaining_accounts_info: Option<RemainingAccountsInfo>
  }
}

impl PackData for CollectRewardV2Params {
  const LEN_BOUNDS: LenBounds = LenBounds::new(2, Some(8));

  fn len(&self) -> usize {
    if self.remaining_accounts_info.is_none() { 2 } else { 8 }
  }

  fn pack_data(&self, buffer: &mut Vector<u8>) -> crate::Result<()> {
    let Self { reward_index, remaining_accounts_info } = self;
    reward_index.pack_data(buffer)?;
    remaining_accounts_info.pack_data(buffer)?;
    Ok(())
  }

  fn unpack_data(bytes: &[u8]) -> crate::Result<Self> {
    Ok(Self {
      reward_index: u8::unpack_data(sub_slice(bytes, 0..1))?,
      remaining_accounts_info: Option::<_>::unpack_data(sub_slice(bytes, 1..))?,
    })
  }
}

impl TestingInstances for CollectRewardV2Params {
  fn min_instance() -> crate::Result<Self> {
    Ok(Self { reward_index: 1, remaining_accounts_info: None })
  }

  fn variable_instance() -> crate::Result<Self> {
    Ok(Self {
      reward_index: 1,
      remaining_accounts_info: Some(RemainingAccountsInfo {
        slices: [RemainingAccountsSlice {
          accounts_type: AccountsType::SupplementalTickArrays,
          length: 1,
        }],
      }),
    })
  }
}
