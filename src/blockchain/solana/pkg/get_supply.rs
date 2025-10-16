#[wtx::pkg(
  data_format(json_rpc("getSupply")),
  id(crate::blockchain::solana::SolanaId),
  transport(http)
)]
pub(crate) mod pkg {
  use crate::blockchain::solana::{
    Commitment, HttpPkgsAux, JsonRpcResponseResultWithContext, SolanaAddressHashStr,
  };
  use wtx::collection::Vector;

  #[pkg::aux]
  impl<A, DRSR> HttpPkgsAux<A, DRSR> {}

  #[derive(Debug, serde::Serialize)]
  #[pkg::req_data]
  pub struct GetSupplyReq(
    #[pkg::field(name = "conf")]
    #[serde(skip_serializing_if = "Option::is_none")]
    Option<GetSupplyConfig>,
  );

  #[pkg::res_data]
  pub type GetSupplyRes = JsonRpcResponseResultWithContext<GetSupply>;

  #[derive(Debug, serde::Deserialize)]
  #[doc = _generic_res_data_elem_doc!()]
  #[serde(rename_all = "camelCase")]
  pub struct GetSupply {
    /// Total supply in lamports
    pub total: u64,
    /// Circulating supply in lamports
    pub circulating: u64,
    /// Non-circulating supply in lamports
    pub non_circulating: u64,
    /// An array of account addresses of non-circulating accounts, as strings.
    pub non_circulating_accounts: Vector<SolanaAddressHashStr>,
  }

  #[derive(Debug, serde::Serialize)]
  #[doc = generic_config_doc!()]
  #[serde(rename_all = "camelCase")]
  pub struct GetSupplyConfig {
    #[doc = commitment_doc!()]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub commitment: Option<Commitment>,
    /// Exclude non circulating accounts list from response
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exclude_non_circulating_accounts_list: Option<bool>,
  }
}
