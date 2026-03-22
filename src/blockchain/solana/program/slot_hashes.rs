use crate::blockchain::solana::SolanaAddressHash;
use core::ops::Deref;
use wtx::collection::Vector;

pub(crate) const MAX_ENTRIES: usize = 512; // about 2.5 minutes to get your vote in

/// A tuple representing a slot and its hash.
pub type SlotHash = (u64, SolanaAddressHash);

/// A collection of recent slot hashes.
#[derive(Debug, Default)]
pub struct SlotHashes(Vector<SlotHash>);

impl SlotHashes {
  /// Returns the index of the given slot in the collection.
  pub fn position(&self, slot: &u64) -> Option<usize> {
    self.binary_search_by(|(probe, _)| slot.cmp(probe)).ok()
  }
}

impl Deref for SlotHashes {
  type Target = Vector<SlotHash>;

  fn deref(&self) -> &Self::Target {
    &self.0
  }
}
