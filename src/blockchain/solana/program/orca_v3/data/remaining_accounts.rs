use crate::blockchain::solana::{
  misc::sub_slice,
  program::{LenBounds, PackData, TestingInstances},
};
use wtx::collection::Vector;

/// Identifies the category of extra accounts in a Whirlpool instruction.
#[derive(Debug, Eq, PartialEq)]
pub enum AccountsType {
  /// Transfer hook for token A.
  TransferHookA,
  /// Transfer hook for token B.
  TransferHookB,
  /// Transfer hook for reward tokens.
  TransferHookReward,
  /// Transfer hook for input accounts.
  TransferHookInput,
  /// Transfer hook for intermediate accounts.
  TransferHookIntermediate,
  /// Transfer hook for output accounts.
  TransferHookOutput,
  /// General supplemental tick arrays.
  SupplementalTickArrays,
  /// First set of supplemental tick arrays.
  SupplementalTickArraysOne,
  /// Second set of supplemental tick arrays.
  SupplementalTickArraysTwo,
}

create_data_struct! {
  #[derive(Debug, Eq, PartialEq)]
  /// Information about supplemental accounts used in Whirlpool instructions.
  pub struct RemainingAccountsInfo<> {
    /// An array containing information about each slice of remaining accounts.
    pub slices: [RemainingAccountsSlice; 1],
  }
}

/// Metadata representing a contiguous group of accounts of the same type.
#[derive(Debug, Eq, PartialEq)]
pub struct RemainingAccountsSlice {
  /// The type of accounts in this slice.
  pub accounts_type: AccountsType,
  /// The number of accounts in this slice.
  pub length: u8,
}

impl PackData for RemainingAccountsInfo {
  const LEN_BOUNDS: LenBounds = LenBounds::new(6, Some(6));

  fn pack_data(&self, buffer: &mut Vector<u8>) -> crate::Result<()> {
    let Self { slices: [RemainingAccountsSlice { accounts_type, length }] } = self;
    u32::pack_data(&1, buffer)?;
    u8::pack_data(
      &match accounts_type {
        AccountsType::TransferHookA => 0,
        AccountsType::TransferHookB => 1,
        AccountsType::TransferHookReward => 2,
        AccountsType::TransferHookInput => 3,
        AccountsType::TransferHookIntermediate => 4,
        AccountsType::TransferHookOutput => 5,
        AccountsType::SupplementalTickArrays => 6,
        AccountsType::SupplementalTickArraysOne => 7,
        AccountsType::SupplementalTickArraysTwo => 8,
      },
      buffer,
    )?;
    u8::pack_data(length, buffer)?;
    Ok(())
  }

  fn unpack_data(bytes: &[u8]) -> crate::Result<Self> {
    let accounts_type = match u8::unpack_data(sub_slice(bytes, 4..5))? {
      0 => AccountsType::TransferHookA,
      1 => AccountsType::TransferHookB,
      2 => AccountsType::TransferHookReward,
      3 => AccountsType::TransferHookInput,
      4 => AccountsType::TransferHookIntermediate,
      5 => AccountsType::TransferHookOutput,
      6 => AccountsType::SupplementalTickArrays,
      7 => AccountsType::SupplementalTickArraysOne,
      8 => AccountsType::SupplementalTickArraysTwo,
      _ => return Err(crate::Error::SolanaInvalidAccountData),
    };
    Ok(Self {
      slices: [RemainingAccountsSlice {
        accounts_type,
        length: u8::unpack_data(sub_slice(bytes, 5..6))?,
      }],
    })
  }
}

impl TestingInstances for RemainingAccountsInfo {
  fn min_instance() -> crate::Result<Self> {
    Ok(Self {
      slices: [RemainingAccountsSlice {
        accounts_type: AccountsType::SupplementalTickArrays,
        length: 2,
      }; 1],
    })
  }
}
