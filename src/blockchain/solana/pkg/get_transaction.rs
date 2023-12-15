#[wtx_macros::pkg(
  api(crate::blockchain::solana::Solana),
  data_format(json_rpc("getTransaction")),
  error(crate::Error),
  transport(http)
)]
pub(crate) mod pkg {
  use crate::blockchain::solana::{
    Commitment, SolanaHttpPkgsAux, TransactionEncoding, TransactionOutput,
  };
  use wtx::misc::AsyncBounds;

  #[pkg::aux]
  impl<DRSR> SolanaHttpPkgsAux<DRSR> {}

  #[derive(Debug, serde::Serialize)]
  #[pkg::req_data]
  pub struct GetTransactionReq<S>(
    #[pkg::field(name = "hash")] S,
    #[pkg::field(name = "config")]
    #[serde(skip_serializing_if = "Option::is_none")]
    Option<GetTransactionConfig>,
  )
  where
    S: AsyncBounds;

  #[pkg::res_data]
  pub type GetTransactionRes = TransactionOutput;

  #[derive(Debug, serde::Serialize)]
  #[doc = generic_config_doc!()]
  #[serde(rename_all = "camelCase")]
  pub struct GetTransactionConfig {
    #[doc = commitment_doc!()]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub commitment: Option<Commitment>,
    /// Transaction encoding
    #[serde(skip_serializing_if = "Option::is_none")]
    pub encoding: Option<TransactionEncoding>,
    /// Maximum transaction version
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_supported_transaction_version: Option<u8>,
  }
}
