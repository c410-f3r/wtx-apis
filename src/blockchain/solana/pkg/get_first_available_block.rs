#[wtx_macros::pkg(
  data_format(json_rpc("getFirstAvailableBlock")),
  id(crate::blockchain::solana::SolanaId),
  transport(http)
)]
pub(crate) mod pkg {
  use crate::blockchain::solana::HttpPkgsAux;

  #[pkg::aux]
  impl<A, DRSR> HttpPkgsAux<A, DRSR> {}

  #[derive(Debug, serde::Serialize)]
  #[pkg::req_data]
  pub struct GetFirstAvailableBlockReq;

  #[pkg::res_data]
  pub type GetFirstAvailableBlockRes = u64;
}
