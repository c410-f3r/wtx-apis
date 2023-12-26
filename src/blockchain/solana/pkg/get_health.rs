#[wtx_macros::pkg(
  api(crate::blockchain::solana::Solana),
  data_format(json_rpc("getHealth")),
  transport(http)
)]
pub(crate) mod pkg {
  use crate::blockchain::solana::HttpPkgsAux;

  #[pkg::aux]
  impl<A, DRSR> HttpPkgsAux<A, DRSR> {}

  #[derive(Debug, serde::Serialize)]
  #[pkg::req_data]
  pub struct GetHealthReq;

  #[pkg::res_data]
  pub type GetHealthRes = GetHealthOk;

  /// `getHealth` response that represents a successful request.
  #[derive(Debug, serde::Deserialize)]
  #[serde(rename_all = "lowercase")]
  pub enum GetHealthOk {
    /// Ok
    Ok,
  }
}
