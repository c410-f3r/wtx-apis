#[wtx::pkg(
  data_format(json_rpc("eth_getLogs")),
  id(crate::blockchain::ethereum::EthereumId),
  transport(http)
)]
pub(crate) mod pkg {
  use crate::blockchain::ethereum::{Filter, HttpPkgsAux, Log};
  use wtx::collection::Vector;

  #[pkg::aux]
  impl<A, DRSR> HttpPkgsAux<A, DRSR> {}

  #[derive(Debug, serde::Serialize)]
  #[pkg::req_data]
  pub struct EthGetLogsReq<'filter>(#[pkg::field(name = "filter")] &'filter Filter);

  #[pkg::res_data]
  pub type EthGetLogsRes = Option<Vector<Log>>;
}
