use starknet_types_core::{felt::Felt, hash::Poseidon};

/// A stateful hasher for Starknet Poseidon hash.
///
/// Using this hasher is the same as calling [`poseidon_hash_many`].
#[derive(Debug, Default, Clone)]
pub struct PoseidonHasher {
  state: [Felt; 3],
  buffer: Option<Felt>,
}

impl PoseidonHasher {
  /// Creates a new [`PoseidonHasher`].
  pub fn new() -> Self {
    Self::default()
  }

  /// Absorbs message into the hash.
  pub fn update(&mut self, msg: Felt) {
    match self.buffer.take() {
      Some(previous_message) => {
        self.state[0] += previous_message;
        self.state[1] += msg;
        Poseidon::hades_permutation(&mut self.state);
      }
      None => {
        self.buffer = Some(msg);
      }
    }
  }

  /// Finishes and returns hash.
  pub fn finalize(mut self) -> Felt {
    // Applies padding
    match self.buffer.take() {
      Some(last_message) => {
        self.state[0] += last_message;
        self.state[1] += Felt::ONE;
      }
      None => {
        self.state[0] += Felt::ONE;
      }
    }
    Poseidon::hades_permutation(&mut self.state);

    self.state[0]
  }
}
