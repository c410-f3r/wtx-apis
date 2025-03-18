/// Mercado pago response
#[derive(Debug, serde::Deserialize)]
#[serde(rename_all = "camelCase", untagged)]
pub enum MercadoPagoResponse<S, T> {
  /// Success
  Ok(T),
  /// Error
  Err(MercadoPagoError<S>),
}

/// Mercado pago error
#[derive(Debug, serde::Deserialize)]
pub struct MercadoPagoError<T> {
  /// Error
  pub error: T,
  /// Message
  pub message: T,
  /// Status
  pub status: u16,
}

impl<S, T> MercadoPagoResponse<S, T> {
  /// Result of the standard library
  #[inline]
  pub fn into_rslt(self) -> Result<T, MercadoPagoError<S>> {
    match self {
      MercadoPagoResponse::Ok(elem) => Ok(elem),
      MercadoPagoResponse::Err(mercado_pago_error) => Err(mercado_pago_error),
    }
  }
}
