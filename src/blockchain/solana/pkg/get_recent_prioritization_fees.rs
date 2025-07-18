#[wtx::pkg(
  data_format(json_rpc("getRecentPrioritizationFees")),
  id(crate::blockchain::solana::SolanaId),
  transport(http)
)]
pub(crate) mod pkg {
  use crate::blockchain::solana::HttpPkgsAux;
  use serde::Serialize;
  use wtx::collection::Vector;

  #[pkg::aux]
  impl<A, DRSR> HttpPkgsAux<A, DRSR> {}

  #[derive(Debug, serde::Serialize)]
  #[pkg::req_data]
  pub struct GetRecentPrioritizationFeesReq<S>(
    #[pkg::field(name = "array")]
    #[serde(serialize_with = "crate::misc::serialize_as_tuple")]
    #[serde(skip_serializing_if = "Option::is_none")]
    S,
  )
  where
    S: Serialize;

  #[pkg::res_data]
  pub type GetRecentPrioritizationFeesRes = Vector<GetRecentPrioritizationFees>;

  #[derive(Debug, PartialEq, serde::Deserialize)]
  #[doc = _generic_res_data_elem_doc!()]
  pub struct GetRecentPrioritizationFees {
    /// Slot in which sample was taken at
    slot: u64,
    /// The per-compute-unit fee paid by at least one successfully landed transaction, specified in
    /// increments of 0.000001 lamports
    prioritization_fee: u64,
  }
}
