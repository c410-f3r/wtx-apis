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
  pub struct SwapV2Params<> {
    /// The amount of input or output token to swap from (depending on amount_specified_is_input).
    pub amount: u64,
    /// The maximum/minimum of input/output token to swap into (depending on amount_specified_is_input).
    pub other_amount_threshold: u64,
    /// The maximum/minimum price the swap will swap to.
    pub sqrt_price_limit: u128,
    /// Specifies the token the parameter `amount`represents. If true, the amount represents the input token of the swap.
    pub amount_specified_is_input: bool,
    /// The direction of the swap. True if swapping from A to B. False if swapping from B to A.
    pub a_to_b: bool,
    /// Remaining accounts from the tick calculation
    pub remaining_accounts_info: Option<RemainingAccountsInfo>
  }
}

impl PackData for SwapV2Params {
  const LEN_BOUNDS: LenBounds = LenBounds::new(35, Some(41));

  fn len(&self) -> usize {
    if self.remaining_accounts_info.is_none() { 35 } else { 41 }
  }

  fn pack_data(&self, buffer: &mut Vector<u8>) -> crate::Result<()> {
    let Self {
      amount,
      other_amount_threshold,
      sqrt_price_limit,
      amount_specified_is_input,
      a_to_b,
      remaining_accounts_info,
    } = self;
    amount.pack_data(buffer)?;
    other_amount_threshold.pack_data(buffer)?;
    sqrt_price_limit.pack_data(buffer)?;
    amount_specified_is_input.pack_data(buffer)?;
    a_to_b.pack_data(buffer)?;
    remaining_accounts_info.pack_data(buffer)?;
    Ok(())
  }

  fn unpack_data(bytes: &[u8]) -> crate::Result<Self> {
    Ok(Self {
      amount: u64::unpack_data(sub_slice(bytes, 0..8))?,
      other_amount_threshold: u64::unpack_data(sub_slice(bytes, 8..16))?,
      sqrt_price_limit: u128::unpack_data(sub_slice(bytes, 16..32))?,
      amount_specified_is_input: bool::unpack_data(sub_slice(bytes, 32..33))?,
      a_to_b: bool::unpack_data(sub_slice(bytes, 33..34))?,
      remaining_accounts_info: Option::<_>::unpack_data(sub_slice(bytes, 34..))?,
    })
  }
}

impl TestingInstances for SwapV2Params {
  fn min_instance() -> crate::Result<Self> {
    Ok(Self {
      amount: 1,
      other_amount_threshold: 2,
      sqrt_price_limit: 3,
      amount_specified_is_input: false,
      a_to_b: true,
      remaining_accounts_info: None,
    })
  }

  fn variable_instance() -> crate::Result<Self> {
    Ok(Self {
      amount: 1,
      other_amount_threshold: 2,
      sqrt_price_limit: 3,
      amount_specified_is_input: false,
      a_to_b: true,
      remaining_accounts_info: Some(RemainingAccountsInfo {
        slices: [RemainingAccountsSlice {
          accounts_type: AccountsType::SupplementalTickArrays,
          length: 1,
        }],
      }),
    })
  }
}
