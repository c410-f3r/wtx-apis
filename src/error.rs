#[allow(unused_imports, reason = "optional features")]
use alloc::{boxed::Box, string::String};
use core::fmt::{Debug, Display, Formatter};
use wtx::http::StatusCode;

/// All possible errors are grouped here
#[derive(Debug)]
pub enum Error {
  // External
  //
  /// See [bincode::Error].
  #[cfg(feature = "solana")]
  Bincode(bincode::Error),
  /// See [`rmp_serde::encode::Error`].
  #[cfg(feature = "rmp-serde")]
  RmpSerdeEncode(rmp_serde::encode::Error),
  /// See [`signature::Error`].
  #[cfg(feature = "signature")]
  Signature(signature::Error),
  /// See [`serde_json::Error`].
  #[cfg(feature = "serde_json")]
  SerdeJson(serde_json::Error),
  /// See [`wtx::Error`].
  Wtx(wtx::Error),

  // Ethereum
  //
  /// A sequence of bytes has a length greater than 32
  BytesAreGreaterThanWord,
  /// Indices couldn't identify decoding bytes
  UnknownDecodingBytes,
  /// A word points to unknown bytes
  WordIdxDoesNotHaveCorrespondingBytes,

  // Internal
  //
  /// An submitted transaction could not be confirmed by an external actor.
  CouldNotConfirmTransaction,
  /// Request was expecting a different HTTP status code.
  IncompatibleStatusCode(StatusCode, StatusCode),

  // Mercado Pago
  //
  /// It wasn't possible to fetch an access token
  UnableToGetAccessToken,
  /// Mercado Pago error
  #[cfg(feature = "mercado-pago")]
  MercadoPagoError(Box<crate::payment_gateway::mercado_pago::MercadoPagoError<String>>),

  // Olist
  //
  /// Olist error
  #[cfg(feature = "olist")]
  OlistError(crate::erp::olist::OlistError<String>),
  /// Unexpected token response
  OlistUnexpectedTokenResponse,

  // Solana
  //
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
  #[cfg(feature = "solana")]
  /// Transaction error
  SolanaTxError(crate::blockchain::solana::TransactionError),

  // SuperFrete
  //
  /// SuperFrete error
  #[cfg(feature = "super-frete")]
  SuperFreteError(Box<crate::carrier::super_frete::SuperFreteError<String>>),
}

impl core::error::Error for Error {}

// External

#[cfg(feature = "solana")]
impl From<bincode::Error> for Error {
  #[inline]
  fn from(from: bincode::Error) -> Self {
    Self::Bincode(from)
  }
}

#[cfg(feature = "signature")]
impl From<signature::Error> for Error {
  #[inline]
  fn from(from: signature::Error) -> Self {
    Self::Signature(from)
  }
}

#[cfg(feature = "rmp-serde")]
impl From<rmp_serde::encode::Error> for Error {
  #[inline]
  fn from(from: rmp_serde::encode::Error) -> Self {
    Self::RmpSerdeEncode(from)
  }
}

#[cfg(feature = "serde_json")]
impl From<serde_json::Error> for Error {
  #[inline]
  fn from(from: serde_json::Error) -> Self {
    Self::SerdeJson(from)
  }
}

impl From<wtx::Error> for Error {
  #[inline]
  fn from(from: wtx::Error) -> Self {
    Self::Wtx(from)
  }
}

// Mercado Pago

#[cfg(feature = "mercado-pago")]
impl<S> From<crate::payment_gateway::mercado_pago::MercadoPagoError<S>> for Error
where
  S: wtx::misc::Lease<str>,
{
  #[inline]
  fn from(from: crate::payment_gateway::mercado_pago::MercadoPagoError<S>) -> Self {
    Self::MercadoPagoError(
      crate::payment_gateway::mercado_pago::MercadoPagoError {
        error: from.error.lease().into(),
        message: from.message.lease().into(),
        status: from.status,
      }
      .into(),
    )
  }
}

// Olist

#[cfg(feature = "olist")]
impl From<crate::erp::olist::OlistError<String>> for Error {
  #[inline]
  fn from(from: crate::erp::olist::OlistError<String>) -> Self {
    Self::OlistError(from)
  }
}

// SuperFrete

#[cfg(feature = "super-frete")]
impl<S> From<crate::carrier::super_frete::SuperFreteError<S>> for Error
where
  S: wtx::misc::Lease<str>,
{
  #[inline]
  fn from(from: crate::carrier::super_frete::SuperFreteError<S>) -> Self {
    Self::SuperFreteError(
      crate::carrier::super_frete::SuperFreteError { message: from.message.lease().into() }.into(),
    )
  }
}

impl Display for Error {
  #[inline]
  fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
    <Error as Debug>::fmt(self, f)
  }
}
