#[wtx_macros::pkg(
  api(crate::blockchain::solana::Solana),
  data_format(json_rpc("getSlotLeaders")),
  transport(http)
)]
pub(crate) mod pkg {
  use crate::blockchain::solana::{HttpPkgsAux, SolanaAddressHashStr};
  use alloc::vec::Vec;

  #[pkg::aux]
  impl<A, DRSR> HttpPkgsAux<A, DRSR> {}

  #[derive(Debug, serde::Serialize)]
  #[pkg::req_data]
  pub struct GetSlotLeadersReq(#[pkg::field(name = "start")] u64, #[pkg::field(name = "len")] u64);

  #[pkg::res_data]
  pub type GetSlotLeadersRes = Vec<SolanaAddressHashStr>;
}
