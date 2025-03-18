#[wtx_macros::pkg(
  data_format(json_rpc("getIdentity")),
  id(crate::blockchain::solana::SolanaId),
  transport(http)
)]
pub(crate) mod pkg {
  use crate::{blockchain::solana::HttpPkgsAux, misc::MaxAddressHashStr};

  #[pkg::aux]
  impl<A, DRSR> HttpPkgsAux<A, DRSR> {}

  #[derive(Debug, serde::Serialize)]
  #[pkg::req_data]
  pub struct GetIdentityReq;

  #[derive(Debug, serde::Deserialize)]
  #[pkg::res_data]
  #[serde(rename_all = "lowercase")]
  pub struct GetIdentityRes {
    /// Base58 address
    pub identity: MaxAddressHashStr,
  }
}
