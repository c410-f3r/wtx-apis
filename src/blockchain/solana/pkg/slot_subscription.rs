#[wtx_macros::pkg(
  data_format(json_rpc("slotSubscribe")),
  id(crate::blockchain::solana::SolanaId),
  transport(ws)
)]
pub(crate) mod sub {
  use crate::blockchain::solana::WsPkgsAux;

  #[pkg::aux]
  impl<A, DRSR> WsPkgsAux<A, DRSR> {}

  #[derive(Debug, serde::Serialize)]
  #[pkg::req_data]
  pub struct SlotSubscribeReq;

  #[pkg::res_data]
  pub type SlotSubscribeRes = u64;
}

#[wtx_macros::pkg(
  data_format(json_rpc("slotUnsubscribe")),
  id(crate::blockchain::solana::SolanaId),
  transport(ws)
)]
pub(crate) mod unsub {
  use crate::blockchain::solana::WsPkgsAux;

  #[pkg::aux]
  impl<A, DRSR> WsPkgsAux<A, DRSR> {}

  #[derive(Debug, serde::Serialize)]
  #[pkg::req_data]
  pub struct SlotUnsubscribeReq(
    #[serde(serialize_with = "crate::misc::serialize_as_tuple")]
    #[pkg::field(name = "id")]
    u64,
  );

  #[pkg::res_data]
  pub type SlotUnsubscribeeRes = bool;
}
