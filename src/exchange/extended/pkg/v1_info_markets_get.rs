#[wtx::pkg(data_format(json), id(crate::exchange::extended::ExtendedId), transport(http))]
pub(crate) mod pkg {
  use crate::exchange::extended::{Extended, HttpPkgsAux, HttpResponse, Market};
  use wtx::{
    client_api_framework::network::{HttpParams, transport::TransportParams},
    collection::Vector,
  };

  #[pkg::aux]
  impl<A, DRSR> HttpPkgsAux<A, DRSR> {}

  #[pkg::before_sending]
  async fn before_sending(
    api: &mut Extended,
    params: V1InfoMarketsGetParams<'_>,
    trans_params: &mut HttpParams,
  ) -> crate::Result<()> {
    api.auth_req(format_args!("/v1/info/markets"), trans_params)?;
    if !params.is_empty() {
      let _ = trans_params.ext_req_params_mut().rrb.uri.query_writer_many("market", params, ',')?;
    }
    Ok(())
  }

  #[pkg::params]
  pub type V1InfoMarketsGetParams<'any> = &'any [&'any str];

  #[derive(Debug, serde::Serialize)]
  #[pkg::req_data]
  pub struct V1InfoMarketsGetReq;

  #[pkg::res_data]
  pub type V1InfoMarketsGetRes<'de> = HttpResponse<Vector<Market>>;
}
