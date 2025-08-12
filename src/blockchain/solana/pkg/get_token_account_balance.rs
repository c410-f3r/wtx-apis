#[wtx::pkg(
  data_format(json_rpc("getTokenAccountBalance")),
  id(crate::blockchain::solana::SolanaId),
  transport(http)
)]
pub(crate) mod pkg {
  use crate::blockchain::solana::{
    Commitment, HttpPkgsAux, JsonRpcResponseResultWithContext, program::spl_token::AccountBalance,
  };

  #[pkg::aux]
  impl<A, DRSR> HttpPkgsAux<A, DRSR> {}

  #[derive(Debug, serde::Serialize)]
  #[pkg::req_data]
  pub struct GetTokenAccountBalanceReq<S>(
    #[pkg::field(name = "pk")] S,
    #[pkg::field(name = "config")]
    #[serde(skip_serializing_if = "Option::is_none")]
    Option<GetTokenAccountBalanceConfig>,
  );

  #[pkg::res_data]
  pub type GetTokenAccountBalanceRes = JsonRpcResponseResultWithContext<AccountBalance>;

  #[derive(Debug, serde::Serialize)]
  #[doc = generic_config_doc!()]
  pub struct GetTokenAccountBalanceConfig {
    #[doc = commitment_doc!()]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub commitment: Option<Commitment>,
  }
}
