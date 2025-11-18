use k256::U256;

/// An Ethereum ECDSA signature.
#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct Signature {
  pub(crate) y_parity: bool,
  pub(crate) r: U256,
  pub(crate) s: U256,
}

impl Signature {
  #[cfg(test)]
  pub(crate) fn all_bytes(&self) -> [u8; 65] {
    let mut rslt = [0; 65];
    rslt[..32].copy_from_slice(&self.r.to_be_bytes());
    rslt[32..64].copy_from_slice(&self.s.to_be_bytes());
    rslt[64] = self.v();
    rslt
  }

  pub(crate) fn v(&self) -> u8 {
    27u8.wrapping_add(self.y_parity.into())
  }
}

impl From<(k256::ecdsa::Signature, k256::ecdsa::RecoveryId)> for Signature {
  fn from(value: (k256::ecdsa::Signature, k256::ecdsa::RecoveryId)) -> Self {
    let r = U256::from_be_slice(value.0.r().to_bytes().as_ref());
    let s = U256::from_be_slice(value.0.s().to_bytes().as_ref());
    Self { y_parity: value.1.is_y_odd(), r, s }
  }
}
