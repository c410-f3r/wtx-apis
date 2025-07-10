#[wtx::pkg(
  data_format(json_rpc("getGenesisHash")),
  id(crate::blockchain::solana::SolanaId),
  transport(http)
)]
pub(crate) mod pkg {
  use crate::{blockchain::solana::HttpPkgsAux, misc::MaxBlockHashStr};

  #[pkg::aux]
  impl<A, DRSR> HttpPkgsAux<A, DRSR> {}

  #[derive(Debug, serde::Serialize)]
  #[pkg::req_data]
  pub struct GetGenesisHashReq;

  #[pkg::res_data]
  pub type GetGenesisHashRes = MaxBlockHashStr;
}
