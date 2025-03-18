#[wtx_macros::pkg(
  data_format(json_rpc("rootSubscribe")),
  id(crate::blockchain::solana::SolanaId),
  transport(ws)
)]
pub(crate) mod sub {
  use crate::blockchain::solana::WsPkgsAux;

  #[pkg::aux]
  impl<A, DRSR> WsPkgsAux<A, DRSR> {}

  #[derive(Debug, serde::Serialize)]
  #[pkg::req_data]
  pub struct RootSubscribeReq;

  #[pkg::res_data]
  pub type RootSubscribeRes = u64;
}

#[wtx_macros::pkg(
  data_format(json_rpc("rootUnsubscribe")),
  id(crate::blockchain::solana::SolanaId),
  transport(ws)
)]
pub(crate) mod unsub {
  use crate::blockchain::solana::WsPkgsAux;

  #[pkg::aux]
  impl<A, DRSR> WsPkgsAux<A, DRSR> {}

  #[derive(Debug, serde::Serialize)]
  #[pkg::req_data]
  pub struct RootUnsubscribeReq(
    #[serde(serialize_with = "crate::misc::serialize_as_tuple")]
    #[pkg::field(name = "id")]
    u64,
  );

  #[pkg::res_data]
  pub type RootUnsubscribeRes = bool;
}
