use wtx::misc::Vector;

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
      let content = serde::__private::de::Content::deserialize(deserializer)?;
      let content_ref = serde::__private::de::ContentRefDeserializer::<DE::Error>::new(&content);
      if let Ok(elem) = T::deserialize(content_ref) {
        return Ok(OlistResult { result: Ok(elem) });
      }
      Ok(OlistResult { result: Err(<OlistError<STR> as Deserialize>::deserialize(content_ref)?) })
    }
  }
}
