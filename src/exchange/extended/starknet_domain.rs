use crate::exchange::extended::{PoseidonHasher, cairo_short_string_to_felt};
use starknet_types_core::felt::Felt;

#[derive(Debug)]
pub struct StarknetDomain<S> {
  pub chain_id: S,
  pub name: S,
  pub revision: u32,
  pub version: S,
}

impl<S> StarknetDomain<S>
where
  S: AsRef<str>,
{
  // "StarknetDomain"("name":"shortstring","version":"shortstring","chainId":"shortstring","revision":"shortstring")
  pub const SELECTOR: Felt = Felt::from_raw([
    45164882192052528,
    3320515356094353366,
    7437117071726711362,
    6953663458211852539,
  ]);

  pub fn hash(&self) -> Felt {
    let mut hasher = PoseidonHasher::new();
    hasher.update(Self::SELECTOR);
    hasher.update(cairo_short_string_to_felt(self.name.as_ref()));
    hasher.update(cairo_short_string_to_felt(self.version.as_ref()));
    hasher.update(cairo_short_string_to_felt(self.chain_id.as_ref()));
    hasher.update(self.revision.into());
    let hash = hasher.finalize();
    return hash;
  }
}
