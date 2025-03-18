#[wtx_macros::pkg(
  data_format(json_rpc("getClusterNodes")),
  id(crate::blockchain::solana::SolanaId),
  transport(http)
)]
pub(crate) mod pkg {
  use crate::blockchain::solana::{HttpPkgsAux, SolanaAddressHashStr};
  use wtx::misc::{ArrayString, Vector};

  #[pkg::aux]
  impl<A, DRSR> HttpPkgsAux<A, DRSR> {}

  #[derive(Debug, serde::Serialize)]
  #[pkg::req_data]
  pub struct GetClusterNodesReq;

  #[pkg::res_data]
  pub type GetClusterNodesRes = Vector<GetClusterNodes>;

  #[derive(Debug, PartialEq, serde::Deserialize)]
  #[doc = _generic_res_data_elem_doc!()]
  pub struct GetClusterNodes {
    /// Gossip network address.
    pub gossip: ArrayString<21>,
    /// Node Base58 public key.
    pub pubkey: SolanaAddressHashStr,
    /// JSON RPC network address of the node.
    pub rpc: Option<ArrayString<32>>,
    /// TPU network address.
    pub tpu: Option<ArrayString<21>>,
    /// The software version of the node.
    pub version: Option<ArrayString<16>>,
  }
}
