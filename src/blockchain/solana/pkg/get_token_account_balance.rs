#[wtx_macros::pkg(
  api(crate::blockchain::solana::Solana),
  data_format(json_rpc("getTokenAccountBalance")),
  transport(http)
)]
pub(crate) mod pkg {
  use crate::blockchain::solana::{
    program::spl_token::AccountBalance, Commitment, HttpPkgsAux, JsonRpcResponseResultWithContext,
  };
  use wtx::misc::AsyncBounds;

  #[pkg::aux]
  impl<A, DRSR> HttpPkgsAux<A, DRSR> {}

  #[derive(Debug, serde::Serialize)]
  #[pkg::req_data]
  pub struct GetTokenAccountBalanceReq<S>(
    #[pkg::field(name = "pk")] S,
    #[pkg::field(name = "config")]
    #[serde(skip_serializing_if = "Option::is_none")]
    Option<GetTokenAccountBalanceConfig>,
  )
  where
    S: AsyncBounds;

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
