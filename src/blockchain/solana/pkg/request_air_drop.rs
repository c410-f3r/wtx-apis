#[wtx::pkg(
  data_format(json_rpc("requestAirDrop")),
  id(crate::blockchain::solana::SolanaId),
  transport(http)
)]
pub(crate) mod pkg {
  use crate::blockchain::solana::{Commitment, HttpPkgsAux, SolanaSignatureHashStr};

  #[pkg::aux]
  impl<A, DRSR> HttpPkgsAux<A, DRSR> {}

  #[derive(Debug, serde::Serialize)]
  #[pkg::req_data]
  pub struct RequestAirDropReq<S>(
    #[pkg::field(name = "address")] S,
    #[pkg::field(name = "lamports")] u64,
    #[pkg::field(name = "conf")]
    #[serde(skip_serializing_if = "Option::is_none")]
    Option<RequestAirDropConfig>,
  );

  #[pkg::res_data]
  pub type RequestAirDropRes = SolanaSignatureHashStr;

  #[derive(Debug, serde::Serialize)]
  #[doc = generic_config_doc!()]
  #[serde(rename_all = "camelCase")]
  pub struct RequestAirDropConfig {
    #[doc = commitment_doc!()]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub commitment: Option<Commitment>,
  }
}
