#[wtx_macros::pkg(
  api(crate::blockchain::ethereum::Ethereum),
  data_format(json_rpc("eth_getBalance")),
  transport(http)
)]
pub(crate) mod pkg {
  use crate::blockchain::ethereum::{BlockNumber, HttpPkgsAux};
  use ethereum_types::U256;

  #[pkg::aux]
  impl<A, DRSR> HttpPkgsAux<A, DRSR> {}

  #[derive(Debug, serde::Serialize)]
  #[pkg::req_data]
  pub struct EthGetBalanceReq<'any, S>(
    #[pkg::field(name = "addr")] S,
    #[pkg::field(name = "block_number")] &'any BlockNumber,
  );

  #[pkg::res_data]
  pub type EthGetBalanceRes = Option<U256>;
}
