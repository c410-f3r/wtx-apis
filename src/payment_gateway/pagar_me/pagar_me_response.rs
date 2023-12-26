use crate::payment_gateway::pagar_me::pagar_me_error::PagarMeErrors;
use serde::{de::Deserializer, Deserialize};

/// Replied from an issued [crate::data_format::GraphQlRequest].
#[derive(Debug)]
pub struct PagarMeResponse<T> {
  /// Content depends if request was successful or not.
  pub result: Result<T, PagarMeErrors>,
}

impl<'de, T> Deserialize<'de> for PagarMeResponse<T>
where
  T: Deserialize<'de>,
{
  #[inline]
  fn deserialize<DE>(deserializer: DE) -> Result<Self, DE::Error>
  where
    DE: Deserializer<'de>,
  {
    #[derive(Debug, serde::Deserialize)]
    #[serde(untagged)]
    enum Response<T> {
      T(T),
      PagarMeErrors(PagarMeErrors),
    }

    Ok(Self {
      result: match <_>::deserialize(deserializer)? {
        Response::T(elem) => Ok(elem),
        Response::PagarMeErrors(elem) => Err(elem),
      },
    })
  }
}
