use crate::blockchain::solana::{SolanaSignatureHash, VersionedMessageInput};
use wtx::collection::{ArrayWrapper, Vector};
#[cfg(feature = "ed25519-dalek")]
use {
  crate::blockchain::solana::SolanaBlockhash,
  ed25519_dalek::{Signer, SigningKey},
};

/// Transport format suitable for user input.
#[derive(Debug, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct TransactionInput {
  #[serde(with = "crate::blockchain::solana::short_vec")]
  /// Signatures
  pub signatures: Vector<SolanaSignatureHash>,
  /// Message
  pub message: VersionedMessageInput,
}

impl TransactionInput {
  /// Takes all the necessary parameters to validate and transform data into a suitable format for
  /// submission.
  #[cfg(feature = "ed25519-dalek")]
  pub fn new<'sk>(
    buffer: &mut Vector<u8>,
    blockhash: SolanaBlockhash,
    message: VersionedMessageInput,
    sk: impl Clone + IntoIterator<Item = &'sk SigningKey>,
  ) -> crate::Result<Self> {
    let mut this = Self { signatures: <_>::default(), message };
    let VersionedMessageInput::V0(message) = &mut this.message;
    if blockhash != message.recent_blockhash {
      message.recent_blockhash = blockhash;
    }
    this._set_empty_signatures()?;
    this.do_sign(buffer, sk)?;
    Ok(this)
  }

  /// Checks if all signatures are actually signed.
  pub fn check_signatures(&self) -> crate::Result<()> {
    let default = [0; 64];
    let mut filled: usize = 0;
    let all_are_signed = self.signatures.iter().all(|signature| {
      let is_signed = signature.as_slice() != &default;
      if is_signed {
        filled = filled.wrapping_add(1);
      }
      is_signed
    });
    if all_are_signed {
      Ok(())
    } else {
      let len = self.signatures.len();
      Err(crate::Error::SolanaSignersShouldHaveSignedAllTransactionSignatures(filled, len))
    }
  }

  /// Signs or re-signs the contained message with the provided `blockhash` and `keypairs`.
  #[cfg(feature = "ed25519-dalek")]
  pub fn sign<'sk>(
    &mut self,
    blockhash: SolanaBlockhash,
    buffer: &mut Vector<u8>,
    sk: impl Clone + IntoIterator<Item = &'sk SigningKey>,
  ) -> crate::Result<()> {
    let VersionedMessageInput::V0(message) = &mut self.message;
    if blockhash != message.recent_blockhash {
      message.recent_blockhash = blockhash;
      self.signatures.iter_mut().for_each(|signature| *signature = [0; 64].into());
    }
    self.do_sign(buffer, sk)?;
    Ok(())
  }

  #[cfg(feature = "ed25519-dalek")]
  fn do_sign<'sk>(
    &mut self,
    mut buffer: &mut Vector<u8>,
    sk: impl Clone + IntoIterator<Item = &'sk SigningKey>,
  ) -> crate::Result<()> {
    buffer.clear();
    bincode::serialize_into(&mut buffer, &self.message)?;
    let signing_keypair_positions = {
      let VersionedMessageInput::V0(message) = &self.message;
      let signed_keys = message
        .account_keys
        .get(0..message.header.num_required_signatures.into())
        .unwrap_or_default();
      sk.clone().into_iter().map(|keypair| {
        signed_keys.iter().position(|signed_key| keypair.verifying_key().as_ref() == signed_key)
      })
    };
    for (opt, keypair) in signing_keypair_positions.zip(sk) {
      let signature = keypair.try_sign(buffer.as_ref())?.to_bytes();
      let signature_mut = match opt.and_then(|idx| self.signatures.get_mut(idx)) {
        None => {
          return Err(crate::Error::SolanaInexistentOrOutOfBoundsSignatureIndex(
            self.signatures.len(),
            opt,
          ));
        }
        Some(elem) => elem,
      };
      *signature_mut = signature.into();
    }
    self.check_signatures()?;
    buffer.clear();
    Ok(())
  }

  fn _set_empty_signatures(&mut self) -> crate::Result<()> {
    let VersionedMessageInput::V0(message) = &self.message;
    let len: usize = message.header.num_required_signatures.into();
    self.signatures.clear();
    for _ in 0..len {
      self.signatures.push(ArrayWrapper([0; 64]))?;
    }
    Ok(())
  }
}
