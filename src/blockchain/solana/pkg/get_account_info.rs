#[wtx_macros::pkg(
  data_format(json_rpc("getAccountInfo")),
  id(crate::blockchain::solana::SolanaId),
  transport(http)
)]
pub(crate) mod pkg {
  use crate::blockchain::solana::{
    Account, AccountEncoding, Commitment, DataSlice, HttpPkgsAux, JsonRpcResponseResultWithContext,
  };

  #[pkg::aux]
  impl<A, DRSR> HttpPkgsAux<A, DRSR> {}

  #[derive(Debug, serde::Serialize)]
  #[pkg::req_data]
  pub struct GetAccountInfoReq<S>(
    #[pkg::field(name = "pk")] S,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[pkg::field(name = "config")]
    Option<GetAccountInfoConfig>,
  );

  #[pkg::res_data]
  pub type GetAccountInfoRes = JsonRpcResponseResultWithContext<Option<Account>>;

  #[derive(Debug, serde::Serialize)]
  #[doc = generic_config_doc!()]
  #[serde(rename_all = "camelCase")]
  pub struct GetAccountInfoConfig {
    /// Account encoding.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub encoding: Option<AccountEncoding>,
    #[doc = commitment_doc!()]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub commitment: Option<Commitment>,
    /// Data slice.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_slice: Option<DataSlice>,
  }
}
