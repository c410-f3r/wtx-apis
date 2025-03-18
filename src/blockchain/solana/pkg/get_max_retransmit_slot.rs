#[wtx_macros::pkg(
  data_format(json_rpc("getMaxRetransmitSlot")),
  id(crate::blockchain::solana::SolanaId),
  transport(http)
)]
pub(crate) mod pkg {
  use crate::blockchain::solana::HttpPkgsAux;

  #[derive(Debug, serde::Serialize)]
  #[pkg::req_data]
  pub struct GetMaxRetransmitSlotReq;

  #[pkg::aux]
  impl<A, DRSR> HttpPkgsAux<A, DRSR> {}

  #[pkg::res_data]
  pub type GetMaxRetransmitSlotRes = u64;
}
