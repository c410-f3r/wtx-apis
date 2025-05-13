#[wtx_macros::pkg(
  data_format(json_rpc("getTokenAccountsByOwner")),
  id(crate::blockchain::solana::SolanaId),
  transport(http)
)]
pub(crate) mod pkg {
  use crate::blockchain::solana::{
    Account, AccountEncoding, Commitment, DataSlice, HttpPkgsAux, JsonRpcResponseResultWithContext,
    MintOrProgramId, SolanaAddressHashStr,
  };
  use wtx::collection::Vector;

  #[pkg::aux]
  impl<A, DRSR> HttpPkgsAux<A, DRSR> {}

  #[derive(Debug, serde::Serialize)]
  #[pkg::req_data]
  pub struct GetTokenAccountsByOwnerReq<S>(
    #[pkg::field(name = "pk")] S,
    #[pkg::field(name = "criteria")] MintOrProgramId<S>,
    #[pkg::field(name = "config")]
    #[serde(skip_serializing_if = "Option::is_none")]
    Option<GetTokenAccountsByOwnerConfig>,
  );

  #[pkg::res_data]
  pub type GetTokenAccountsByOwnerRes =
    JsonRpcResponseResultWithContext<Vector<GetTokenAccountsByOwner>>;

  #[derive(Debug, serde::Serialize)]
  #[doc = generic_config_doc!()]
  #[serde(rename_all = "camelCase")]
  pub struct GetTokenAccountsByOwnerConfig {
    /// Account encoding
    #[serde(skip_serializing_if = "Option::is_none")]
    pub encoding: Option<AccountEncoding>,
    #[doc = commitment_doc!()]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub commitment: Option<Commitment>,
    #[doc = min_context_slot_doc!()]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_context_slot: Option<u64>,
    /// Data slice
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_slice: Option<DataSlice>,
  }

  #[derive(Debug, serde::Deserialize)]
  #[doc = _generic_res_data_elem_doc!()]
  pub struct GetTokenAccountsByOwner {
    /// Account
    pub account: Account,
    /// Base58 identifier.
    pub pubkey: SolanaAddressHashStr,
  }
}
