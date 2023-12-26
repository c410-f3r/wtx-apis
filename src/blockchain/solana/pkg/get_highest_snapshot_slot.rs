#[wtx_macros::pkg(
  api(crate::blockchain::solana::Solana),
  data_format(json_rpc("getHighestSnapshotSlot")),
  transport(http)
)]
pub(crate) mod pkg {
  use crate::blockchain::solana::HttpPkgsAux;

  #[pkg::aux]
  impl<A, DRSR> HttpPkgsAux<A, DRSR> {}

  #[derive(Debug, serde::Serialize)]
  #[pkg::req_data]
  pub struct GetHighestSnapshotSlotReq;

  #[derive(Debug, PartialEq, serde::Deserialize)]
  #[pkg::res_data]
  pub struct GetHighestSnapshotSlotRes {
    /// Highest full snapshot slot.
    pub full: u64,
    /// Highest incremental snapshot slot based on full.
    pub incremental: Option<u64>,
  }
}
