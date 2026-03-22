use crate::{
  blockchain::solana::{
    misc::sub_slice,
    program::{LenBounds, PackData, address_lookup_table::data::NewAddresses},
  },
  misc::HashArray32Unit,
};
use wtx::collection::Vector;

create_data_enum! {
  #[derive(Debug, Eq, PartialEq)]
  /// Program instructions for managing address lookup tables.
  pub enum Instruction {
    /// Closes an empty lookup table and reclaims its lamports.
    CloseLookupTable,
    /// Creates a new address lookup table.
    CreateLookupTable((u64, u8)),
    /// Deactivates a lookup table so it can eventually be closed.
    DeactivateLookupTable,
    /// Appends a list of addresses to a lookup table.
    ExtendLookupTable(NewAddresses<HashArray32Unit>),
  }
}

impl PackData for Instruction {
  const LEN_BOUNDS: LenBounds = LenBounds::new(4, None);

  fn len(&self) -> usize {
    let instance_len = match self {
      Self::CloseLookupTable | Self::DeactivateLookupTable => 0,
      Self::CreateLookupTable(el) => el.len(),
      Self::ExtendLookupTable(el) => el.len(),
    };
    4usize.saturating_add(instance_len)
  }

  fn pack_data(&self, buffer: &mut Vector<u8>) -> crate::Result<()> {
    match self {
      Self::CloseLookupTable => {
        4u32.pack_data(buffer)?;
      }
      Self::CreateLookupTable(elem) => {
        0u32.pack_data(buffer)?;
        elem.pack_data(buffer)?;
      }
      Self::DeactivateLookupTable => {
        3u32.pack_data(buffer)?;
      }
      Self::ExtendLookupTable(elem) => {
        2u32.pack_data(buffer)?;
        elem.pack_data(buffer)?;
      }
    }
    Ok(())
  }

  fn unpack_data(bytes: &[u8]) -> crate::Result<Self> {
    Ok(match u32::unpack_data(sub_slice(bytes, 0..4))? {
      0 => Self::CreateLookupTable(<_>::unpack_data(sub_slice(bytes, 4..13))?),
      3 => Self::DeactivateLookupTable,
      2 => Self::ExtendLookupTable(<_>::unpack_data(sub_slice(bytes, 4..))?),
      4 => Self::CloseLookupTable,
      _ => return Err(crate::Error::SolanaInvalidAccountData),
    })
  }
}
