use crate::{
  blockchain::solana::{
    misc::sub_slice,
    program::{LenBounds, PackData, address_lookup_table::data::LookupTableMeta},
  },
  misc::HashArray32Unit,
};
use wtx::collection::Vector;

create_data_enum! {
  #[allow(clippy::large_enum_variant)]
  #[derive(Debug, Eq, PartialEq)]
  pub enum ProgramState {
    Uninitialized,
    LookupTable(LookupTableMeta<HashArray32Unit>),
  }
}

impl PackData for ProgramState {
  const LEN_BOUNDS: LenBounds = LenBounds::new(4, Some(56));

  fn len(&self) -> usize {
    let n = match self {
      ProgramState::Uninitialized => 0,
      ProgramState::LookupTable(el) => el.len(),
    };
    n.saturating_add(4)
  }

  fn pack_data(&self, buffer: &mut Vector<u8>) -> crate::Result<()> {
    match self {
      ProgramState::Uninitialized => 0u32.pack_data(buffer)?,
      ProgramState::LookupTable(el) => {
        1u32.pack_data(buffer)?;
        el.pack_data(buffer)?;
      }
    }
    Ok(())
  }

  fn unpack_data(bytes: &[u8]) -> crate::Result<Self> {
    Ok(match u32::unpack_data(sub_slice(bytes, ..4))? {
      0 => Self::Uninitialized,
      1 => Self::LookupTable(<_>::unpack_data(sub_slice(bytes, 4..))?),
      _ => return Err(crate::Error::SolanaInvalidAccountData),
    })
  }
}
