#[wtx_macros::pkg(
  api(crate::blockchain::solana::Solana),
  data_format(json_rpc("getMaxRetransmitSlot")),
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
