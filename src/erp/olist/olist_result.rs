use wtx::collection::Vector;

/// Wrapper structure for contact response
#[derive(Debug, ::serde::Deserialize)]
pub struct OlistError<S> {
  /// Details
  pub detalhes: Option<Vector<OlistErrorDetails<S>>>,
  /// Message
  pub mensagem: S,
}

/// Wrapper structure for contact response
#[derive(Debug, ::serde::Deserialize)]
pub struct OlistErrorDetails<S> {
  /// Field
  pub campo: S,
  /// Message
  pub mensagem: S,
}

/// Wrapper structure for contact response
#[derive(Debug)]
pub struct OlistResult<T, STR> {
  /// Result
  pub result: Result<T, OlistError<STR>>,
}

mod serde {
  use crate::erp::olist::{OlistError, OlistResult};
  use serde::{Deserialize, de::Deserializer};

  impl<'de, T, STR> Deserialize<'de> for OlistResult<T, STR>
  where
    STR: Deserialize<'de>,
    T: Deserialize<'de>,
  {
    #[inline]
    fn deserialize<DE>(deserializer: DE) -> Result<OlistResult<T, STR>, DE::Error>
    where
      DE: Deserializer<'de>,
    {
      #[derive(Deserialize)]
      #[serde(untagged)]
      enum Helper<T, STR> {
        Success(T),
        Error(OlistError<STR>),
      }
      let helper = Helper::<T, STR>::deserialize(deserializer)?;
      let result = match helper {
        Helper::Success(elem) => Ok(elem),
        Helper::Error(err) => Err(err),
      };
      Ok(OlistResult { result })
    }
  }
}
