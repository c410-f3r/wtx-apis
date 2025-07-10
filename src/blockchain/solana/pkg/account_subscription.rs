#[wtx::pkg(
  data_format(json_rpc("accountSubscribe")),
  id(crate::blockchain::solana::SolanaId),
  transport(ws)
)]
pub(crate) mod sub {
  use crate::blockchain::solana::{AccountEncoding, Commitment, WsPkgsAux};

  #[pkg::aux]
  impl<A, DRSR> WsPkgsAux<A, DRSR> {}

  #[derive(Debug, serde::Serialize)]
  #[pkg::req_data]
  pub struct AccountSubscribeReq<S>(
    #[pkg::field(name = "pk")] S,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[pkg::field(name = "config")]
    Option<AccountSubscribeConfig>,
  );

  #[pkg::res_data]
  pub type AccountSubscribeRes = u64;

  #[derive(Debug, serde::Serialize)]
  #[doc = generic_config_doc!()]
  pub struct AccountSubscribeConfig {
    /// Account encoding.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub encoding: Option<AccountEncoding>,
    #[doc = commitment_doc!()]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub commitment: Option<Commitment>,
  }
}

#[wtx::pkg(
  data_format(json_rpc("accountUnsubscribe")),
  id(crate::blockchain::solana::SolanaId),
  transport(ws)
)]
pub(crate) mod unsub {
  use crate::blockchain::solana::WsPkgsAux;

  #[pkg::aux]
  impl<A, DRSR> WsPkgsAux<A, DRSR> {}

  #[derive(Debug, serde::Serialize)]
  #[pkg::req_data]
  pub struct AccountUnsubscribeReq(
    #[serde(serialize_with = "crate::misc::serialize_as_tuple")]
    #[pkg::field(name = "id")]
    u64,
  );

  #[pkg::res_data]
  pub type AccountUnsubscribeRes = bool;
}
