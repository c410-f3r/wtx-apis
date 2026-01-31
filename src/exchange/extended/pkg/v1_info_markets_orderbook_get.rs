#[wtx::pkg(data_format(json), id(crate::exchange::extended::ExtendedId), transport(http))]
pub(crate) mod pkg {
  use crate::exchange::extended::{
    Extended, HttpPkgsAux, HttpResponse, OrderBookResponse, OrderBookResponseLevel,
  };
  use wtx::{client_api_framework::network::HttpParams, collection::Vector};

  #[pkg::aux]
  impl<A, DRSR> HttpPkgsAux<A, DRSR> {}

  #[pkg::before_sending]
  async fn before_sending(
    api: &mut Extended,
    params: V1InfoMarketsOrderbookGetParams<'_>,
    trans_params: &mut HttpParams,
  ) -> crate::Result<()> {
    api.auth_req(format_args!("/v1/info/markets/{params}/orderbook"), trans_params)?;
    Ok(())
  }

  #[pkg::params]
  pub type V1InfoMarketsOrderbookGetParams<'any> = &'any str;

  #[derive(Debug, serde::Serialize)]
  #[pkg::req_data]
  pub struct V1InfoMarketsOrderbookGetReq;

  #[pkg::res_data]
  pub type V1InfoMarketsOrderbookGetRes<'de> =
    HttpResponse<OrderBookResponse<Vector<OrderBookResponseLevel>, Vector<OrderBookResponseLevel>>>;
}
