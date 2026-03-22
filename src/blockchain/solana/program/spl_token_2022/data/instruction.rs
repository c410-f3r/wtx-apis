use crate::blockchain::solana::{
  misc::sub_slice,
  program::{
    LenBounds, PackData,
    spl_token_2022::data::{TransferCheckedParams, TransferParams},
  },
};
use wtx::collection::Vector;

create_data_enum! {
  #[derive(Debug, Eq, PartialEq)]
  pub enum Instruction {
    CloseAccount,
    GetAccountDataSize,
    InitializeAccount,
    SyncNative,
    Transfer(TransferParams),
    TransferChecked(TransferCheckedParams),
  }
}

impl PackData for Instruction {
  const LEN_BOUNDS: LenBounds = LenBounds::new(1, Some(10));

  fn len(&self) -> usize {
    let instance_len = match self {
      Instruction::CloseAccount
      | Instruction::GetAccountDataSize
      | Instruction::InitializeAccount
      | Instruction::SyncNative => 0,
      Instruction::Transfer(el) => el.len(),
      Instruction::TransferChecked(el) => el.len(),
    };
    instance_len.saturating_add(1)
  }

  fn pack_data(&self, buffer: &mut Vector<u8>) -> crate::Result<()> {
    match self {
      Self::InitializeAccount => 1u8.pack_data(buffer)?,
      Self::Transfer(params) => {
        3u8.pack_data(buffer)?;
        params.pack_data(buffer)?;
      }
      Self::CloseAccount => 9u8.pack_data(buffer)?,
      Self::TransferChecked(params) => {
        12u8.pack_data(buffer)?;
        params.pack_data(buffer)?;
      }
      Self::SyncNative => 17u8.pack_data(buffer)?,
      Self::GetAccountDataSize => 21u8.pack_data(buffer)?,
    }
    Ok(())
  }

  fn unpack_data(bytes: &[u8]) -> crate::Result<Self> {
    Ok(match bytes.first() {
      Some(&1) => Self::InitializeAccount,
      Some(&3) => Self::Transfer(<_>::unpack_data(sub_slice(bytes, 1..9))?),
      Some(&9) => Self::CloseAccount,
      Some(&12) => Self::TransferChecked(<_>::unpack_data(sub_slice(bytes, 1..10))?),
      Some(&17) => Self::SyncNative,
      Some(&21) => Self::GetAccountDataSize,
      _ => return Err(crate::Error::SolanaInvalidAccountData),
    })
  }
}
