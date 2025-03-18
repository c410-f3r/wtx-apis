#[wtx_macros::pkg(
  data_format(json_rpc("eth_estimateGas")),
  id(crate::blockchain::ethereum::EthereumId),
  transport(http)
)]
pub(crate) mod pkg {
  use crate::blockchain::ethereum::{BlockNumber, CallRequest, HttpPkgsAux};
  use ethereum_types::U256;

  #[pkg::aux]
  impl<A, DRSR> HttpPkgsAux<A, DRSR> {}

  #[derive(Debug, serde::Serialize)]
  #[pkg::req_data]
  pub struct EthEstimateGasReq<'any>(
    #[pkg::field(name = "block_number")]
    #[serde(skip_serializing_if = "Option::is_none")]
    Option<&'any BlockNumber>,
    #[pkg::field(name = "cr")] &'any CallRequest,
  );

  #[pkg::res_data]
  pub type EthEstimateGasRes = U256;
}
