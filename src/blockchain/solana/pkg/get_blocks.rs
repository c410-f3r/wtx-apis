#[wtx_macros::pkg(
  data_format(json_rpc("getBlocks")),
  id(crate::blockchain::solana::SolanaId),
  transport(http)
)]
pub(crate) mod pkg {
  use crate::blockchain::solana::{Commitment, HttpPkgsAux};
  use wtx::collection::Vector;

  #[pkg::aux]
  impl<A, DRSR> HttpPkgsAux<A, DRSR> {}

  #[derive(Debug, serde::Serialize)]
  #[pkg::req_data]
  pub struct GetBlocksReq(
    #[pkg::field(name = "start_slot")] u64,
    #[pkg::field(name = "end_slot")]
    #[serde(skip_serializing_if = "Option::is_none")]
    Option<u64>,
    #[pkg::field(name = "config")]
    #[serde(skip_serializing_if = "Option::is_none")]
    Option<GetBlocksConfig>,
  );

  #[pkg::res_data]
  pub type GetBlocksRes = Vector<u64>;

  #[derive(Debug, serde::Serialize)]
  #[doc = generic_config_doc!()]
  pub struct GetBlocksConfig {
    #[doc = commitment_doc!()]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub commitment: Option<Commitment>,
  }
}
