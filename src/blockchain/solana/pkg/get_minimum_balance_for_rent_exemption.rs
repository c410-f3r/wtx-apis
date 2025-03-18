#[wtx_macros::pkg(
  data_format(json_rpc("getMinimumBalanceForRentExemption")),
  id(crate::blockchain::solana::SolanaId),
  transport(http)
)]
pub(crate) mod pkg {
  use crate::blockchain::solana::{Commitment, HttpPkgsAux};

  #[pkg::aux]
  impl<A, DRSR> HttpPkgsAux<A, DRSR> {}

  #[derive(Debug, serde::Serialize)]
  #[pkg::req_data]
  pub struct GetMinimumBalanceForRentExemptionReq(
    #[pkg::field(name = "data_len")] usize,
    #[pkg::field(name = "config")]
    #[serde(skip_serializing_if = "Option::is_none")]
    Option<GetMinimumBalanceForRentExemptionConfig>,
  );

  #[pkg::res_data]
  pub type GetMinimumBalanceForRentExemptionRes = u64;

  #[derive(Debug, serde::Serialize)]
  #[doc = generic_config_doc!()]
  pub struct GetMinimumBalanceForRentExemptionConfig {
    #[doc = commitment_doc!()]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub commitment: Option<Commitment>,
  }
}
