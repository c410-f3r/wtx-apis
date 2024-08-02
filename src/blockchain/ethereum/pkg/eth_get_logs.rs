#[wtx_macros::pkg(
  api(crate::blockchain::ethereum::Ethereum),
  data_format(json_rpc("eth_getLogs")),
  transport(http)
)]
pub(crate) mod pkg {
  use crate::blockchain::ethereum::{Filter, HttpPkgsAux, Log};
  use alloc::vec::Vec;

  #[pkg::aux]
  impl<A, DRSR> HttpPkgsAux<A, DRSR> {}

  #[derive(Debug, serde::Serialize)]
  #[pkg::req_data]
  pub struct EthGetLogsReq<'filter>(#[pkg::field(name = "filter")] &'filter Filter);

  #[pkg::res_data]
  pub type EthGetLogsRes = Option<Vec<Log>>;
}
