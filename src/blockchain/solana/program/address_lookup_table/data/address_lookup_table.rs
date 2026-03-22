//! Address lookup tables.

use crate::{
  blockchain::solana::{
    AddressLookupTableAccount, SolanaAddressHash,
    program::{SlotHashes, address_lookup_table::data::LookupTableMeta},
  },
  misc::{Hash, HashArray32Unit},
};
use alloc::borrow::Cow;
use wtx::{collection::Vector, misc::Wrapper};

/// The fixed byte size of the address lookup table metadata.
pub const LOOKUP_TABLE_META_SIZE: usize = 56;

/// Represents the data within an address lookup table account.
#[derive(Debug, PartialEq, Eq)]
pub struct AddressLookupTable<'any, H> {
  /// Metadata for the lookup table.
  pub meta: LookupTableMeta<H>,
  /// List of addresses stored in the table.
  pub addresses: Cow<'any, [HashArray32Unit]>,
}

impl<'any, H> AddressLookupTable<'any, H> {
  /// Converts the table into an `AddressLookupTableAccount` for transaction processing.
  pub fn into_alta(self, key: SolanaAddressHash) -> crate::Result<AddressLookupTableAccount> {
    Ok(AddressLookupTableAccount {
      key,
      addresses: self
        .addresses
        .iter()
        .map(Hash::bytes)
        .copied()
        .collect::<Wrapper<wtx::Result<Vector<_>>>>()
        .0?,
    })
  }
}

impl<'any> AddressLookupTable<'any, HashArray32Unit> {
  /// Returns the number of active addresses in the table for a specific slot.
  pub fn get_active_addresses_len(
    &self,
    current_slot: u64,
    slot_hashes: &SlotHashes,
  ) -> crate::Result<usize> {
    if !self.meta.is_active(current_slot, slot_hashes) {
      return Err(crate::Error::SolanaAddressLookupLookupTableAccountNotFound);
    }
    Ok(if current_slot > self.meta.last_extended_slot {
      self.addresses.len()
    } else {
      self.meta.last_extended_slot_start_index.into()
    })
  }

  /// Retrieves addresses from the table based on the provided indexes.
  pub fn lookup(
    &self,
    current_slot: u64,
    indexes: &[u8],
    slot_hashes: &SlotHashes,
  ) -> crate::Result<Vector<HashArray32Unit>> {
    let active_addresses_len = self.get_active_addresses_len(current_slot, slot_hashes)?;
    let active_addresses = &self.addresses.get(..active_addresses_len).unwrap_or_default();
    let mut rslt = Vector::new();
    for index in indexes {
      rslt.push(
        *active_addresses
          .get(usize::from(*index))
          .ok_or(crate::Error::SolanaAddressLookupInvalidLookupIndex)?,
      )?;
    }
    Ok(rslt)
  }
}

#[cfg(test)]
mod tests {
  use crate::{
    blockchain::solana::program::{
      SlotHashes,
      address_lookup_table::data::{AddressLookupTable, LookupTableMeta},
    },
    misc::HashArray32Unit,
  };
  use alloc::borrow::Cow;
  use wtx::collection::Vector;

  #[test]
  fn test_lookup_from_empty_table() {
    let lookup_table = AddressLookupTable {
      meta: LookupTableMeta::default(),
      addresses: Cow::Owned(Vector::new().into()),
    };
    assert_eq!(lookup_table.lookup(0, &[], &SlotHashes::default()).unwrap(), Vector::new());
    assert!(matches!(
      lookup_table.lookup(0, &[0], &SlotHashes::default()).unwrap_err(),
      crate::Error::SolanaAddressLookupInvalidLookupIndex
    ));
  }

  #[test]
  fn test_lookup_from_table_extended_in_current_slot() {
    let current_slot = 0;
    let addresses =
      alloc::vec![HashArray32Unit::from_bytes([1; 32]), HashArray32Unit::from_bytes([2; 32]),];
    let lookup_table = AddressLookupTable {
      meta: LookupTableMeta {
        last_extended_slot: current_slot,
        last_extended_slot_start_index: 1,
        ..LookupTableMeta::default()
      },
      addresses: Cow::Owned(addresses.clone()),
    };
    assert_eq!(
      lookup_table.lookup(current_slot, &[0], &SlotHashes::default()).unwrap().as_slice(),
      &[addresses[0]]
    );
    assert!(matches!(
      lookup_table.lookup(current_slot, &[1], &SlotHashes::default()).unwrap_err(),
      crate::Error::SolanaAddressLookupInvalidLookupIndex
    ));
  }

  #[test]
  fn test_lookup_from_table_extended_in_previous_slot() {
    let current_slot = 1;
    let addresses = alloc::vec![
      HashArray32Unit::from_bytes([1; 32]),
      HashArray32Unit::from_bytes([2; 32]),
      HashArray32Unit::from_bytes([3; 32]),
      HashArray32Unit::from_bytes([4; 32]),
      HashArray32Unit::from_bytes([5; 32]),
      HashArray32Unit::from_bytes([6; 32]),
      HashArray32Unit::from_bytes([7; 32]),
      HashArray32Unit::from_bytes([8; 32]),
      HashArray32Unit::from_bytes([9; 32]),
      HashArray32Unit::from_bytes([10; 32]),
    ];
    let lookup_table = AddressLookupTable {
      meta: LookupTableMeta {
        last_extended_slot: current_slot - 1,
        last_extended_slot_start_index: 1,
        ..LookupTableMeta::default()
      },
      addresses: Cow::Owned(addresses.clone()),
    };

    assert_eq!(
      lookup_table.lookup(current_slot, &[0, 3, 1, 5], &SlotHashes::default()).unwrap().as_slice(),
      &[addresses[0], addresses[3], addresses[1], addresses[5]]
    );
    assert!(matches!(
      lookup_table.lookup(current_slot, &[10], &SlotHashes::default()).unwrap_err(),
      crate::Error::SolanaAddressLookupInvalidLookupIndex
    ));
  }
}
