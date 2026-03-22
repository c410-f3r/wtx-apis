use crate::exchange::aster::CexSignParams;

/// Structure sent when interacting with single markets
#[derive(Debug, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct MarketReqParams<'any> {
  /// Market pair
  pub symbol: &'any str,
  /// See [`SignParams`].
  #[serde(flatten)]
  pub sign_params: Option<CexSignParams>,
}
