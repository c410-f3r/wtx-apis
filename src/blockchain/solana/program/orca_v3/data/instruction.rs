use crate::blockchain::solana::{
  misc::sub_slice,
  program::{
    LenBounds, PackData,
    orca_v3::data::{
      CollectRewardV2Params, DecreaseLiquidityParams, IncreaseLiquidityParams,
      IncreaseLiquidityV2Params, OpenPositionParams, OpenPositionWithTokenExtensionsParams,
      SwapV2Params,
    },
  },
};
use wtx::collection::Vector;

create_data_enum! {
  #[derive(Debug, Eq, PartialEq)]
  pub enum Instruction {
    ClosePositionWithTokenExtensions,
    CollectFees,
    CollectRewardV2(CollectRewardV2Params),
    DecreaseLiquidity(DecreaseLiquidityParams),
    IncreaseLiquidity(IncreaseLiquidityParams),
    IncreaseLiquidityV2(IncreaseLiquidityV2Params),
    OpenPosition(OpenPositionParams),
    OpenPositionWithTokenExtensions(OpenPositionWithTokenExtensionsParams),
    SwapV2(SwapV2Params),
    UpdateFeesAndRewards,
  }
}

impl PackData for Instruction {
  const LEN_BOUNDS: LenBounds = LenBounds::new(8, Some(49));

  fn len(&self) -> usize {
    let instance_len = match self {
      Instruction::ClosePositionWithTokenExtensions => 0,
      Instruction::CollectFees => 0,
      Instruction::CollectRewardV2(el) => el.len(),
      Instruction::DecreaseLiquidity(el) => el.len(),
      Instruction::IncreaseLiquidity(el) => el.len(),
      Instruction::IncreaseLiquidityV2(el) => el.len(),
      Instruction::OpenPosition(el) => el.len(),
      Instruction::OpenPositionWithTokenExtensions(el) => el.len(),
      Instruction::SwapV2(el) => el.len(),
      Instruction::UpdateFeesAndRewards => 0,
    };
    instance_len.saturating_add(8)
  }

  fn pack_data(&self, buffer: &mut Vector<u8>) -> crate::Result<()> {
    match self {
      Self::ClosePositionWithTokenExtensions => {
        buffer.extend_from_copyable_slice(&[1, 182, 135, 59, 155, 25, 99, 223])?;
      }
      Self::CollectFees => {
        buffer.extend_from_copyable_slice(&[164, 152, 207, 99, 30, 186, 19, 182])?;
      }
      Self::CollectRewardV2(elem) => {
        buffer.extend_from_copyable_slice(&[177, 107, 37, 180, 160, 19, 49, 209])?;
        elem.pack_data(buffer)?;
      }
      Self::DecreaseLiquidity(elem) => {
        buffer.extend_from_copyable_slice(&[160, 38, 208, 111, 104, 91, 44, 1])?;
        elem.pack_data(buffer)?;
      }
      Self::IncreaseLiquidity(elem) => {
        buffer.extend_from_copyable_slice(&[46, 156, 243, 118, 13, 205, 251, 178])?;
        elem.pack_data(buffer)?;
      }
      Self::IncreaseLiquidityV2(elem) => {
        buffer.extend_from_copyable_slice(&[133, 29, 89, 223, 69, 238, 176, 10])?;
        elem.pack_data(buffer)?;
      }
      Self::OpenPosition(elem) => {
        buffer.extend_from_copyable_slice(&[135, 128, 47, 77, 15, 152, 240, 49])?;
        elem.pack_data(buffer)?;
      }
      Self::OpenPositionWithTokenExtensions(elem) => {
        buffer.extend_from_copyable_slice(&[212, 47, 95, 92, 114, 102, 131, 250])?;
        elem.pack_data(buffer)?;
      }
      Self::SwapV2(elem) => {
        buffer.extend_from_copyable_slice(&[43, 4, 237, 11, 26, 201, 30, 98])?;
        elem.pack_data(buffer)?;
      }
      Self::UpdateFeesAndRewards => {
        buffer.extend_from_copyable_slice(&[154, 230, 250, 13, 236, 209, 75, 223])?;
      }
    }
    Ok(())
  }

  fn unpack_data(bytes: &[u8]) -> crate::Result<Self> {
    match *sub_slice(bytes, 0..8) {
      [1, 182, 135, 59, 155, 25, 99, 223] => Ok(Self::ClosePositionWithTokenExtensions),
      [164, 152, 207, 99, 30, 186, 19, 182] => Ok(Self::CollectFees),
      [177, 107, 37, 180, 160, 19, 49, 209] => {
        Ok(Self::CollectRewardV2(<_>::unpack_data(sub_slice(bytes, 8..))?))
      }
      [46, 156, 243, 118, 13, 205, 251, 178] => {
        Ok(Self::IncreaseLiquidity(<_>::unpack_data(sub_slice(bytes, 8..40))?))
      }
      [133, 29, 89, 223, 69, 238, 176, 10] => {
        Ok(Self::IncreaseLiquidityV2(<_>::unpack_data(sub_slice(bytes, 8..40))?))
      }
      [135, 128, 47, 77, 15, 152, 240, 49] => {
        Ok(Self::OpenPosition(<_>::unpack_data(sub_slice(bytes, 8..17))?))
      }
      [160, 38, 208, 111, 104, 91, 44, 1] => {
        Ok(Self::DecreaseLiquidity(<_>::unpack_data(sub_slice(bytes, 8..40))?))
      }
      [212, 47, 95, 92, 114, 102, 131, 250] => {
        Ok(Self::OpenPositionWithTokenExtensions(<_>::unpack_data(sub_slice(bytes, 8..17))?))
      }
      [43, 4, 237, 11, 26, 201, 30, 98] => {
        Ok(Self::SwapV2(<_>::unpack_data(sub_slice(bytes, 8..))?))
      }
      [154, 230, 250, 13, 236, 209, 75, 223] => Ok(Self::UpdateFeesAndRewards),
      _ => Err(crate::Error::SolanaInvalidAccountData),
    }
  }
}
