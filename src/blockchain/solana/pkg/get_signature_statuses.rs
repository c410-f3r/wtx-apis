#[wtx_macros::pkg(
  data_format(json_rpc("getSignatureStatuses")),
  id(crate::blockchain::solana::SolanaId),
  transport(http)
)]
pub(crate) mod pkg {
  use crate::blockchain::solana::{
    Commitment, HttpPkgsAux, JsonRpcResponseResultWithContext, TransactionError,
  };
  use wtx::collection::ArrayVector;

  #[pkg::aux]
  impl<A, DRSR> HttpPkgsAux<A, DRSR> {}

  #[derive(Debug, serde::Serialize)]
  #[pkg::req_data]
  pub struct GetSignatureStatusesReq<S>(
    #[pkg::field(name = "signatures")] S,
    #[pkg::field(name = "config")]
    #[serde(skip_serializing_if = "Option::is_none")]
    Option<GetSignatureStatusesConfig>,
  );

  #[pkg::res_data]
  pub type GetSignatureStatusesRes =
    JsonRpcResponseResultWithContext<ArrayVector<Option<GetSignatureStatuses>, 8>>;

  #[derive(Debug, serde::Serialize)]
  #[doc = generic_config_doc!()]
  #[serde(rename_all = "camelCase")]
  pub struct GetSignatureStatusesConfig {
    search_transaction_history: bool,
  }

  #[derive(Debug, serde::Deserialize)]
  #[doc = _generic_res_data_elem_doc!()]
  #[serde(rename_all = "camelCase")]
  pub struct GetSignatureStatuses {
    /// Commitment
    pub confirmation_status: Commitment,
    /// Number of blocks since signature confirmation.
    pub confirmations: Option<usize>,
    /// Filled if the transaction failed.
    pub err: Option<TransactionError>,
    /// The slot the transaction was processed
    pub slot: u64,
  }
}
