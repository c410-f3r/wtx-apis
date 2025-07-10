#[wtx::pkg(
  data_format(json_rpc("eth_call")),
  id(crate::blockchain::ethereum::EthereumId),
  transport(http)
)]
pub(crate) mod pkg {
  use crate::blockchain::ethereum::{BlockId, CallRequest, HttpPkgsAux};

  #[pkg::aux]
  impl<A, DRSR> HttpPkgsAux<A, DRSR> {}

  #[derive(Debug, serde::Serialize)]
  #[pkg::req_data]
  pub struct EthCallReq<'any>(
    #[pkg::field(name = "block_id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    Option<&'any BlockId>,
    #[pkg::field(name = "cr")] &'any CallRequest,
  );

  #[pkg::res_data]
  pub type EthCallRes = Option<crate::blockchain::ethereum::Bytes>;
}
