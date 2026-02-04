#[wtx::pkg(data_format(json), id(crate::exchange::aster::AsterId), transport(ws))]
pub(crate) mod pkg {
  use crate::exchange::aster::WsPkgsAux;

  #[pkg::aux]
  impl<A, DRSR> WsPkgsAux<A, DRSR> {}

  #[derive(Debug, serde::Serialize)]
  #[pkg::req_data]
  pub struct WsReq<P> {
    /// An identifier established by the Client
    pub id: u64,
    /// A String containing the name of the method to be invoked
    pub method: &'static str,
    /// A Structured value that holds the parameter values to be used during the invocation of the method
    pub params: P,
  }

  #[pkg::res_data]
  #[derive(Debug, serde::Deserialize)]
  pub struct WsRes {
    /// The same value specified in the request.
    pub id: u64,
    /// Result
    pub result: Option<bool>,
  }
}
