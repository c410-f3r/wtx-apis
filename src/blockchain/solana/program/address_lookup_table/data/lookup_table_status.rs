/// Operational state of an address lookup table.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum LookupTableStatus {
  /// The table is active and usable.
  Activated,
  /// The table is being deactivated.
  Deactivating {
    /// Approximate number of blocks remaining until deactivation.
    remaining_blocks: usize,
  },
  /// The table is no longer active.
  Deactivated,
}
