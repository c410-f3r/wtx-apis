/// Cancer order request
#[derive(Debug, serde::Serialize)]
pub struct CancelOrderReqOrder<'any> {
  /// Description
  pub description: &'any str,
  /// Id
  pub id: &'any str,
}
