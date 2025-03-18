#[wtx_macros::pkg(
  data_format(json_rpc("getInflationRate")),
  id(crate::blockchain::solana::SolanaId),
  transport(http)
)]
pub(crate) mod pkg {
  use crate::blockchain::solana::HttpPkgsAux;

  #[pkg::aux]
  impl<A, DRSR> HttpPkgsAux<A, DRSR> {}

  #[derive(Debug, serde::Serialize)]
  #[pkg::req_data]
  pub struct GetInflationRateReq;

  #[derive(Debug, PartialEq, serde::Deserialize)]
  #[pkg::res_data]
  pub struct GetInflationRateRes {
    /// Total inflation
    pub total: f64,
    /// Inflation allocated to validators.
    pub validator: f64,
    /// Inflation allocated to the foundation
    pub foundation: f64,
    /// Epoch for which these values are valid.
    pub epoch: u64,
  }
}
