#[wtx_macros::pkg(
  data_format(json_rpc("getLargestAccounts")),
  id(crate::blockchain::solana::SolanaId),
  transport(http)
)]
pub(crate) mod pkg {
  use crate::blockchain::solana::{
    Commitment, HttpPkgsAux, JsonRpcResponseResultWithContext, SolanaAddressHashStr,
  };
  use serde::Serialize;
  use wtx::misc::Vector;

  #[pkg::aux]
  impl<A, DRSR> HttpPkgsAux<A, DRSR> {}

  #[derive(Debug, serde::Serialize)]
  #[pkg::req_data]
  pub struct GetLargestAccountsReq<S>(
    #[pkg::field(name = "config")]
    #[serde(serialize_with = "crate::misc::serialize_as_tuple")]
    #[serde(skip_serializing_if = "Option::is_none")]
    Option<GetLargestAccountsConfig<S>>,
  )
  where
    S: Serialize;

  #[pkg::res_data]
  pub type GetLargestAccountsRes = JsonRpcResponseResultWithContext<Vector<GetLargestAccounts>>;

  #[derive(Debug, serde::Serialize)]
  #[doc = generic_config_doc!()]
  #[serde(rename_all = "camelCase")]
  pub struct GetLargestAccountsConfig<S> {
    #[doc = commitment_doc!()]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub commitment: Option<Commitment>,
    /// Filter results by account type; currently supported: circulating|nonCirculating
    pub filter: Option<S>,
  }

  #[derive(Debug, PartialEq, serde::Deserialize)]
  #[doc = _generic_res_data_elem_doc!()]
  pub struct GetLargestAccounts {
    /// Base58 identifier
    pub address: SolanaAddressHashStr,
    /// Lamports
    pub lamports: f64,
  }
}
