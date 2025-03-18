#[wtx_macros::pkg(
  data_format(json_rpc("getEpochSchedule")),
  id(crate::blockchain::solana::SolanaId),
  transport(http)
)]
pub(crate) mod pkg {
  use crate::blockchain::solana::HttpPkgsAux;

  #[pkg::aux]
  impl<A, DRSR> HttpPkgsAux<A, DRSR> {}

  #[derive(Debug, serde::Serialize)]
  #[pkg::req_data]
  pub struct GetEpochScheduleReq;

  #[derive(Debug, PartialEq, serde::Deserialize)]
  #[pkg::res_data]
  #[serde(rename_all = "camelCase")]
  pub struct GetEpochScheduleRes {
    /// Maximum number of slots in each epoch.
    pub slots_per_epoch: u64,
    /// Number of slots before beginning of an epoch to calculate a leader schedule for that epoch.
    pub leader_schedule_slot_offset: u64,
    /// Whether epochs start short and grow.
    pub warmup: bool,
    /// First normal-length epoch, log2(slotsPerEpoch) - log2(MINIMUM_SLOTS_PER_EPOCH).
    pub first_normal_epoch: u64,
    /// MINIMUM_SLOTS_PER_EPOCH * (2.pow(firstNormalEpoch) - 1)
    pub first_normal_slot: u64,
  }
}
