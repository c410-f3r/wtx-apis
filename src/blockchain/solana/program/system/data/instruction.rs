use crate::{
  blockchain::solana::{
    misc::sub_slice,
    program::{
      LenBounds, PackData,
      system::data::{CreateAccountParams, CreateAccountWithSeedParams, TransferParams},
    },
  },
  misc::HashArray32Unit,
};
use wtx::collection::Vector;

create_data_enum! {
  #[allow(
    // Upstream determines enum variants
    variant_size_differences
  )]
  #[derive(Debug, Eq, PartialEq)]
  pub enum Instruction {
    CreateAccount(CreateAccountParams<HashArray32Unit>),
    CreateAccountWithSeed(CreateAccountWithSeedParams<HashArray32Unit>),
    Transfer(TransferParams),
  }
}

impl PackData for Instruction {
  const LEN_BOUNDS: LenBounds = LenBounds::new(12, Some(96));

  fn len(&self) -> usize {
    let instance_len = match self {
      Self::CreateAccount(el) => el.len(),
      Self::CreateAccountWithSeed(el) => el.len(),
      Self::Transfer(el) => el.len(),
    };
    4usize.saturating_add(instance_len)
  }

  fn pack_data(&self, buffer: &mut Vector<u8>) -> crate::Result<()> {
    match self {
      Self::CreateAccount(params) => {
        0u32.pack_data(buffer)?;
        params.pack_data(buffer)?;
      }
      Self::Transfer(params) => {
        2u32.pack_data(buffer)?;
        params.pack_data(buffer)?;
      }
      Self::CreateAccountWithSeed(params) => {
        3u32.pack_data(buffer)?;
        params.pack_data(buffer)?;
      }
    }
    Ok(())
  }

  fn unpack_data(bytes: &[u8]) -> crate::Result<Self> {
    Ok(match u32::unpack_data(sub_slice(bytes, 0..4))? {
      0 => Self::CreateAccount(<_>::unpack_data(sub_slice(bytes, 4..52))?),
      2 => Self::Transfer(<_>::unpack_data(sub_slice(bytes, 4..12))?),
      3 => Self::CreateAccountWithSeed(<_>::unpack_data(sub_slice(bytes, 4..))?),
      _ => return Err(crate::Error::SolanaInvalidAccountData),
    })
  }
}
