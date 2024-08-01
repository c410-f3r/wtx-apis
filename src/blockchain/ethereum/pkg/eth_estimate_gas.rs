#[wtx_macros::pkg(
  api(crate::blockchain::ethereum::Ethereum),
  data_format(json_rpc("eth_estimateGas")),
  transport(http)
)]
pub(crate) mod pkg {
  use crate::blockchain::ethereum::{BlockNumber, CallRequest, EthereumHttpPkgsAux};
  use ethereum_types::U256;

  #[pkg::aux]
  impl<DRSR> EthereumHttpPkgsAux<DRSR> {}

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
