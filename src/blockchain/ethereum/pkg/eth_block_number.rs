#[wtx_macros::pkg(
  data_format(json_rpc("eth_blockNumber")),
  id(crate::blockchain::ethereum::EthereumId),
  transport(http)
)]
pub(crate) mod pkg {
  use crate::blockchain::ethereum::HttpPkgsAux;
  use ethereum_types::U64;

  #[pkg::aux]
  impl<A, DRSR> HttpPkgsAux<A, DRSR> {}

  #[derive(Debug, serde::Serialize)]
  #[pkg::req_data]
  pub struct EthBlockNumberReq;

  #[pkg::res_data]
  pub type EthBlockNumberRes = Option<U64>;
}
