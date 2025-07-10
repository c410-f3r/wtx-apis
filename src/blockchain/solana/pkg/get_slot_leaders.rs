#[wtx::pkg(
  data_format(json_rpc("getSlotLeaders")),
  id(crate::blockchain::solana::SolanaId),
  transport(http)
)]
pub(crate) mod pkg {
  use crate::blockchain::solana::{HttpPkgsAux, SolanaAddressHashStr};
  use wtx::collection::Vector;

  #[pkg::aux]
  impl<A, DRSR> HttpPkgsAux<A, DRSR> {}

  #[derive(Debug, serde::Serialize)]
  #[pkg::req_data]
  pub struct GetSlotLeadersReq(#[pkg::field(name = "start")] u64, #[pkg::field(name = "len")] u64);

  #[pkg::res_data]
  pub type GetSlotLeadersRes = Vector<SolanaAddressHashStr>;
}
