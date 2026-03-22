use crate::{
  blockchain::solana::{
    misc::{pack_option_address, sub_slice, unpack_option_address},
    program::{
      LenBounds, PackData, TestingInstances,
      address_lookup_table::data::LookupTableStatus,
      slot_hashes::{MAX_ENTRIES, SlotHashes},
    },
  },
  misc::{Hash, HashArray32ArrayString44, HashArray32Unit},
};
use wtx::collection::Vector;

create_data_struct! {
  #[derive(Debug, PartialEq, Eq)]
  /// Metadata for a Solana Address Lookup Table account.
  pub struct LookupTableMeta<H> {
    /// Slot when the table was deactivated, or `u64::MAX` if active.
    pub deactivation_slot: u64,
    /// Last slot when the table was extended.
    pub last_extended_slot: u64,
    /// Address index where the last extension started.
    pub last_extended_slot_start_index: u8,
    /// Optional authority allowed to manage the lookup table.
    pub authority: Option<H>,
    /// Padding for data alignment.
    pub padding: u16,
  }
}

impl<H> LookupTableMeta<H> {
  /// Returns whether the lookup table is currently active.
  pub fn is_active(&self, current_slot: u64, slot_hashes: &SlotHashes) -> bool {
    match self.status(current_slot, slot_hashes) {
      LookupTableStatus::Activated | LookupTableStatus::Deactivating { .. } => true,
      LookupTableStatus::Deactivated => false,
    }
  }

  /// Returns the current operational status of the lookup table.
  pub fn status(&self, current_slot: u64, slot_hashes: &SlotHashes) -> LookupTableStatus {
    if self.deactivation_slot == u64::MAX {
      LookupTableStatus::Activated
    } else if self.deactivation_slot == current_slot {
      LookupTableStatus::Deactivating { remaining_blocks: MAX_ENTRIES.saturating_add(1) }
    } else if let Some(slot_hash_position) = slot_hashes.position(&self.deactivation_slot) {
      LookupTableStatus::Deactivating {
        remaining_blocks: MAX_ENTRIES.saturating_sub(slot_hash_position),
      }
    } else {
      LookupTableStatus::Deactivated
    }
  }
}

impl PackData for LookupTableMeta<HashArray32Unit> {
  const LEN_BOUNDS: LenBounds = LenBounds::new(20, Some(52));

  fn len(&self) -> usize {
    if self.authority.is_none() { 20 } else { 52 }
  }

  fn pack_data(&self, buffer: &mut Vector<u8>) -> crate::Result<()> {
    let Self {
      deactivation_slot,
      last_extended_slot,
      last_extended_slot_start_index,
      authority,
      padding,
    } = self;
    deactivation_slot.pack_data(buffer)?;
    last_extended_slot.pack_data(buffer)?;
    last_extended_slot_start_index.pack_data(buffer)?;
    pack_option_address(authority.as_ref().map(Hash::bytes), buffer)?;
    padding.pack_data(buffer)?;
    Ok(())
  }

  fn unpack_data(bytes: &[u8]) -> crate::Result<Self> {
    let authority = unpack_option_address(sub_slice(bytes, 17..))?.map(HashArray32Unit::from_bytes);
    let final_range = if authority.is_none() { 18..20 } else { 50..52 };
    Ok(Self {
      deactivation_slot: u64::unpack_data(sub_slice(bytes, 0..8))?,
      last_extended_slot: u64::unpack_data(sub_slice(bytes, 8..16))?,
      last_extended_slot_start_index: u8::unpack_data(sub_slice(bytes, 16..17))?,
      authority,
      padding: u16::unpack_data(sub_slice(bytes, final_range))?,
    })
  }
}

impl TestingInstances for LookupTableMeta<HashArray32Unit> {
  fn min_instance() -> crate::Result<Self> {
    Ok(Self {
      deactivation_slot: 1,
      last_extended_slot: 2,
      last_extended_slot_start_index: 3,
      authority: None,
      padding: 5,
    })
  }

  fn variable_instance() -> crate::Result<Self> {
    Ok(Self {
      deactivation_slot: 1,
      last_extended_slot: 2,
      last_extended_slot_start_index: 3,
      authority: Some(HashArray32Unit::from_bytes([4; 32])),
      padding: 5,
    })
  }
}

impl<H> Default for LookupTableMeta<H> {
  fn default() -> Self {
    Self {
      deactivation_slot: u64::MAX,
      last_extended_slot: 0,
      last_extended_slot_start_index: 0,
      authority: None,
      padding: 0,
    }
  }
}

impl TryFrom<LookupTableMeta<HashArray32Unit>> for LookupTableMeta<HashArray32ArrayString44> {
  type Error = crate::Error;

  fn try_from(from: LookupTableMeta<HashArray32Unit>) -> Result<Self, Self::Error> {
    Ok(Self {
      deactivation_slot: from.deactivation_slot,
      last_extended_slot: from.deactivation_slot,
      last_extended_slot_start_index: from.last_extended_slot_start_index,
      authority: from
        .authority
        .and_then(|el| HashArray32ArrayString44::from_base58_bytes(el.into_bytes()).ok()),
      padding: from.padding,
    })
  }
}
