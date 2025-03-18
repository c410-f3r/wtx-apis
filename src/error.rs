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
  /// See [`cl_aux::Error`].
  ClAux(cl_aux::Error),
  /// See [ed25519_dalek::SignatureError].
  #[cfg(feature = "ed25519-dalek")]
  Ed25519Dalek(ed25519_dalek::SignatureError),
  /// See [ethabi::Error]
  #[cfg(feature = "ethereum")]
  EthAbi(ethabi::Error),
  /// See [primitive_types::Error].
  #[cfg(feature = "ethereum")]
  PrimitiveTypes(primitive_types::Error),
  /// See [`wtx::Error`].
  Wtx(wtx::Error),

  // Ethereum
  //
  /// Bad data serialization
  #[cfg(feature = "ethereum")]
  TokensInvalidOutputType(String),

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

impl From<cl_aux::Error> for Error {
  #[inline]
  fn from(from: cl_aux::Error) -> Self {
    Self::ClAux(from)
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
      crate::carrier::super_frete::SuperFreteError {
        error: from.error.map(|el| el.lease().into()),
        message: from.message.lease().into(),
      }
      .into(),
    )
  }
}

impl Display for Error {
  #[inline]
  fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
    <Error as Debug>::fmt(self, f)
  }
}
