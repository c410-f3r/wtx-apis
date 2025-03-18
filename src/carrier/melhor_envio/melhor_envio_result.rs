/// Melhor envio response
#[derive(Debug, serde::Deserialize)]
#[serde(rename_all = "camelCase", untagged)]
pub enum MelhorEnvioResult<S, T> {
  /// Success
  Ok(T),
  /// Error
  Err(MelhorEnvioError<S>),
}

/// Melhor envio error
#[derive(Debug, serde::Deserialize)]
pub struct MelhorEnvioError<T> {
  /// Message
  pub message: T,
}

impl<S, T> MelhorEnvioResult<S, T> {
  /// Result of the standard library
  #[inline]
  pub fn into_rslt(self) -> Result<T, MelhorEnvioError<S>> {
    match self {
      MelhorEnvioResult::Ok(elem) => Ok(elem),
      MelhorEnvioResult::Err(mercado_pago_error) => Err(mercado_pago_error),
    }
  }
}
