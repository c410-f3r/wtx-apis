#[wtx_macros::pkg(
  api(crate::blockchain::solana::Solana),
  data_format(json_rpc("getStakeMinimumDelegation")),
  transport(http)
)]
pub(crate) mod pkg {
  use crate::blockchain::solana::{Commitment, HttpPkgsAux, JsonRpcResponseResultWithContext};

  #[pkg::aux]
  impl<A, DRSR> HttpPkgsAux<A, DRSR> {}

  #[derive(Debug, serde::Serialize)]
  #[pkg::req_data]
  pub struct GetStakeMinimumDelegationReq(
    #[pkg::field(name = "conf")]
    #[serde(skip_serializing_if = "Option::is_none")]
    Option<GetSignaturesForDelegationConfig>,
  );

  #[pkg::res_data]
  pub type GetStakeMinimumDelegationRes = JsonRpcResponseResultWithContext<u64>;

  #[derive(Debug, serde::Serialize)]
  #[doc = generic_config_doc!()]
  pub struct GetSignaturesForDelegationConfig {
    #[doc = commitment_doc!()]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub commitment: Option<Commitment>,
  }
}
