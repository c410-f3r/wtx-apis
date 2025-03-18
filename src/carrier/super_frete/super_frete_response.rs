/// SuperFrete response
#[derive(Debug, serde::Deserialize)]
#[serde(rename_all = "camelCase", untagged)]
pub enum SuperFreteResponse<S, T> {
  /// Success
  Ok(T),
  /// Error
  Err(SuperFreteError<S>),
}

/// SuperFrete error
#[derive(Debug, serde::Deserialize)]
pub struct SuperFreteError<S> {
  /// Error
  pub error: Option<S>,
  /// Message
  pub message: S,
}

impl<S, T> SuperFreteResponse<S, T> {
  /// Result of the standard library
  #[inline]
  pub fn into_rslt(self) -> Result<T, SuperFreteError<S>> {
    match self {
      SuperFreteResponse::Ok(elem) => Ok(elem),
      SuperFreteResponse::Err(mercado_pago_error) => Err(mercado_pago_error),
    }
  }
}
