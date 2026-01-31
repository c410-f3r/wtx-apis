#[wtx::pkg(data_format(json_rpc("SUBSCRIBE")), id(crate::exchange::aster::AsterId), transport(ws))]
pub(crate) mod sub {
  use crate::exchange::aster::WsPkgsAux;

  #[pkg::aux]
  impl<A, DRSR> WsPkgsAux<A, DRSR> {}

  #[derive(Debug, serde::Serialize)]
  #[pkg::req_data]
  pub struct SubscribeReq<S>(#[pkg::field(name = "streams")] S);

  #[pkg::res_data]
  pub type SubscribeRes = Option<u64>;
}

#[wtx::pkg(
  data_format(json_rpc("UNSUBSCRIBE")),
  id(crate::exchange::aster::AsterId),
  transport(ws)
)]
pub(crate) mod unsub {
  use crate::exchange::aster::WsPkgsAux;

  #[pkg::aux]
  impl<A, DRSR> WsPkgsAux<A, DRSR> {}

  #[derive(Debug, serde::Serialize)]
  #[pkg::req_data]
  pub struct UnsubscribeReq<S>(#[pkg::field(name = "streams")] S);

  #[pkg::res_data]
  pub type UnsubscribeRes = u64;
}
