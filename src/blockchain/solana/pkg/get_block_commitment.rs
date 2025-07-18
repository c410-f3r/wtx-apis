#[wtx::pkg(
  data_format(json_rpc("getBlockCommitment")),
  id(crate::blockchain::solana::SolanaId),
  transport(http)
)]
pub(crate) mod pkg {
  use crate::blockchain::solana::HttpPkgsAux;
  use wtx::collection::Vector;

  #[pkg::aux]
  impl<A, DRSR> HttpPkgsAux<A, DRSR> {}

  #[derive(Debug, serde::Serialize)]
  #[pkg::req_data]
  pub struct GetBlockCommitmentReq(
    #[pkg::field(name = "block")]
    #[serde(serialize_with = "crate::misc::serialize_as_tuple")]
    u64,
  );

  #[pkg::res_data]
  pub type GetBlockCommitmentRes = GetBlockCommitment;

  /// Block commitment
  #[derive(Debug, serde::Deserialize)]
  #[serde(rename_all = "camelCase")]
  pub struct GetBlockCommitment {
    /// Amount of cluster stake in lamports that has voted on the block .
    pub commitment: Option<Vector<u64>>,
    /// Total active stake, in lamports, of the current epoch.
    pub total_stake: u64,
  }
}
