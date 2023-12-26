#[allow(unused_imports)]
use alloc::string::String;
use core::fmt::{Debug, Display, Formatter};
use wtx::client_api_framework::network::StatusCode;

/// All possible errors are grouped here
#[derive(Debug)]
pub enum Error {
  // External
  /// See [bincode::Error].
  #[cfg(feature = "solana")]
  Bincode(bincode::Error),
  /// See [ed25519_dalek::SignatureError].
  #[cfg(feature = "ed25519-dalek")]
  Ed25519Dalek(ed25519_dalek::SignatureError),
  /// See [ethabi::Error]
  #[cfg(feature = "ethereum")]
  EthAbi(ethabi::Error),
  /// See [primitive_types::Error].
  #[cfg(feature = "ethereum")]
  PrimitiveTypes(primitive_types::Error),
  /// See [wtx::Error].
  Wtx(wtx::Error),

  // Aptos
  /// Some endpoints require a minimum set of response headers.
  #[cfg(feature = "aptos")]
  MandatoryResponseHeadersWereNotFound,

  // Ethereum
  /// Bad data serialization
  #[cfg(feature = "ethereum")]
  TokensInvalidOutputType(String),

  // Solana
  /// Returned data from counterpart is everything but a spl-token account
  #[cfg(feature = "solana")]
  SolanaAccountIsNotSplToken,
  /// Returned data from counterpart is everything but a spl-token account mint
  #[cfg(feature = "solana")]
  SolanaAccountIsNotSplTokenMint,
  /// Usually means that no signing public key is available in the list of all public keys
  #[cfg(feature = "solana")]
  SolanaInexistentOrOutOfBoundsSignatureIndex(usize, Option<usize>),
  /// The number of signers is not equal the number os signed signatures
  #[cfg(feature = "solana")]
  SolanaSignersShouldHaveSignedAllTransactionSignatures(usize, usize),
  /// Many collections have a maximum limit of 256 items.
  #[cfg(feature = "solana")]
  SolanaU8Overflow,
  /// A instruction required an account that does not exist
  #[cfg(feature = "solana")]
  SolanaUnknownIxPubKey,
  /// The system only supports v0 messages
  #[cfg(feature = "solana")]
  SolanaUnsupportedMessageFormat,

  // Internal
  /// An submitted transaction could not be confirmed by an external actor.
  CouldNotConfirmTransaction,
  /// For third-party dependencies that throws strings errors
  Generic(&'static str),
  /// Request was expecting a different HTTP status code.
  IncompatibleStatusCode(StatusCode, StatusCode),
}

#[cfg(feature = "solana")]
impl From<bincode::Error> for Error {
  #[inline]
  fn from(from: bincode::Error) -> Self {
    Self::Bincode(from)
  }
}

#[cfg(feature = "ed25519-dalek")]
impl From<ed25519_dalek::SignatureError> for Error {
  #[inline]
  fn from(from: ed25519_dalek::SignatureError) -> Self {
    Self::Ed25519Dalek(from)
  }
}

#[cfg(feature = "ethereum")]
impl From<ethabi::Error> for Error {
  #[inline]
  fn from(from: ethabi::Error) -> Self {
    Self::EthAbi(from)
  }
}

#[cfg(feature = "ethereum")]
impl From<primitive_types::Error> for Error {
  #[inline]
  fn from(from: primitive_types::Error) -> Self {
    Self::PrimitiveTypes(from)
  }
}

impl From<wtx::Error> for Error {
  #[inline]
  fn from(from: wtx::Error) -> Self {
    Self::Wtx(from)
  }
}

impl Display for Error {
  #[inline]
  fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
    <Error as Debug>::fmt(self, f)
  }
}
