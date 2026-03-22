use sha2::{Digest, Sha256};

#[derive(Debug, Default)]
pub(crate) struct Sha256Hasher {
  hasher: Sha256,
}

impl Sha256Hasher {
  pub(crate) fn hash(&mut self, val: &[u8]) {
    self.hasher.update(val);
  }

  pub(crate) fn hashv(&mut self, vals: &[&[u8]]) {
    for val in vals {
      self.hash(val);
    }
  }

  pub(crate) fn result(self) -> crate::Result<[u8; 32]> {
    Ok(self.hasher.finalize().as_slice().try_into().map_err(wtx::Error::from)?)
  }
}
