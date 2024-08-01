#[wtx_macros::pkg(
  api(crate::payment_gateway::pagar_me::PagarMe),
  data_format(json),
  transport(http)
)]
pub(crate) mod pkg {
  use crate::payment_gateway::pagar_me::{PagarMeHttpPkgsAux, PagarMeResponse};
  use wtx::{
    client_api_framework::network::HttpReqParams,
    http::{Header, KnownHeaderName},
  };

  #[pkg::aux]
  impl<DRSR> PagarMeHttpPkgsAux<DRSR> {}

  #[pkg::before_sending]
  async fn before_sending(
    api: &mut crate::payment_gateway::pagar_me::PagarMe,
    params: &mut RecipientBalanceParams<'_>,
    req_params: &mut HttpReqParams,
  ) -> crate::Result<()> {
    api.rt_150.rc.update_params(&api.rt_150.rl).await?;
    req_params.headers.push_front(
      Header {
        is_sensitive: false,
        is_trailer: false,
        name: KnownHeaderName::Authorization.into(),
        value: b"Basic ",
      },
      api.api_key.as_bytes(),
    )?;
    req_params.uri.push_path(format_args!("/recipients/{}/balance", params.recipient_id))?;
    Ok(())
  }

  #[derive(Debug)]
  #[pkg::params]
  pub struct RecipientBalanceParams<'any> {
    /// Specific receptor
    pub recipient_id: &'any str,
  }

  #[derive(Debug, serde::Serialize)]
  #[pkg::req_data]
  #[serde(rename_all = "camelCase")]
  pub struct RecipientBalanceReq;

  #[pkg::res_data]
  pub type RecipientBalanceRes = PagarMeResponse<Balance>;

  /// Amount
  #[derive(Debug, serde::Deserialize)]
  pub struct Amount {
    /// Amount
    pub amount: u64,
  }

  /// Balance
  #[derive(Debug, serde::Deserialize)]
  pub struct Balance {
    /// Available
    pub available: Amount,
    /// Transferred
    pub transferred: Amount,
    /// Waiting funds
    pub waiting_funds: Amount,
  }
}
