#[wtx::pkg(
  data_format(json_rpc("slotsUpdatesSubscribe")),
  id(crate::blockchain::solana::SolanaId),
  transport(ws)
)]
pub(crate) mod sub {
  use crate::blockchain::solana::WsPkgsAux;

  #[pkg::aux]
  impl<A, DRSR> WsPkgsAux<A, DRSR> {}

  #[derive(Debug, serde::Serialize)]
  #[pkg::req_data]
  pub struct SlotsUpdatesSubscribeReq;

  #[pkg::res_data]
  pub type SlotsUpdatesSubscribeRes = u64;
}

#[wtx::pkg(
  data_format(json_rpc("slotsUpdatesUnsubscribe")),
  id(crate::blockchain::solana::SolanaId),
  transport(ws)
)]
pub(crate) mod unsub {
  use crate::blockchain::solana::WsPkgsAux;

  #[pkg::aux]
  impl<A, DRSR> WsPkgsAux<A, DRSR> {}

  #[derive(Debug, serde::Serialize)]
  #[pkg::req_data]
  pub struct SlotsUpdatesUnsubscribeReq(
    #[serde(serialize_with = "crate::misc::serialize_as_tuple")]
    #[pkg::field(name = "id")]
    u64,
  );

  #[pkg::res_data]
  pub type SlotsUpdatesUnsubscribeRes = bool;
}
