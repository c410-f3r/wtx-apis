#[wtx_macros::pkg(
  api(crate::blockchain::solana::Solana),
  data_format(json_rpc("getRecentPerformanceSamples")),
  transport(http)
)]
pub(crate) mod pkg {
  use crate::blockchain::solana::HttpPkgsAux;
  use alloc::vec::Vec;

  #[pkg::aux]
  impl<A, DRSR> HttpPkgsAux<A, DRSR> {}

  #[derive(Debug, serde::Serialize)]
  #[pkg::req_data]
  pub struct GetRecentPerformanceSamplesReq(
    #[pkg::field(name = "limit")]
    #[serde(serialize_with = "crate::misc::serialize_as_tuple")]
    #[serde(skip_serializing_if = "Option::is_none")]
    Option<u64>,
  );

  #[pkg::res_data]
  pub type GetRecentPerformanceSamplesRes = Vec<GetRecentPerformanceSamples>;

  #[derive(Debug, PartialEq, serde::Deserialize)]
  #[doc = _generic_res_data_elem_doc!()]
  #[serde(rename_all = "camelCase")]
  pub struct GetRecentPerformanceSamples {
    /// Slot in which sample was taken at
    slot: u64,
    /// Number of transactions in sample
    num_transactions: u64,
    /// Number of slots in sample
    num_slots: u64,
    /// Number of seconds in a sample window
    sample_period_secs: u16,
  }
}
