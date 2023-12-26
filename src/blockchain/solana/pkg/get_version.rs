#[wtx_macros::pkg(
  api(crate::blockchain::solana::Solana),
  data_format(json_rpc("getVersion")),
  transport(http)
)]
pub(crate) mod pkg {
  use crate::blockchain::solana::HttpPkgsAux;
  use arrayvec::ArrayString;

  #[pkg::aux]
  impl<A, DRSR> HttpPkgsAux<A, DRSR> {}

  #[derive(Debug, serde::Serialize)]
  #[pkg::req_data]
  pub struct GetVersionReq;

  #[derive(Debug, serde::Deserialize)]
  #[serde(rename_all = "kebab-case")]
  #[pkg::res_data]
  pub struct GetVersionRes {
    /// Software version of solana-core.
    pub solana_core: ArrayString<16>,
    /// Unique identifier of the current software's feature set.
    pub feature_set: u64,
  }
}
