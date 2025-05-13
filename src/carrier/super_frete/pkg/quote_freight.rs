#[wtx_macros::pkg(
  data_format(json),
  id(crate::carrier::super_frete::SuperFreteId),
  transport(http)
)]
pub(crate) mod pkg {
  use crate::carrier::super_frete::{
    HttpPkgsAux, QuoteFreightReqGeneric, SuperFrete, SuperFreteResponse,
    misc::manage_token,
    quote_freight_res::{QuoteFreightResGeneric, QuoteFreightResPackage},
  };
  use wtx::{client_api_framework::network::HttpParams, collection::Vector};

  #[pkg::aux]
  impl<A, DRSR> HttpPkgsAux<A, DRSR> {}

  #[pkg::before_sending]
  async fn before_sending(
    api: &mut SuperFrete,
    trans_params: &mut HttpParams,
  ) -> crate::Result<()> {
    manage_token(api, "/api/v0/calculator", trans_params)
  }

  #[pkg::req_data]
  pub type QuoteFreightReq<'any, P> = QuoteFreightReqGeneric<P, &'any str>;

  #[pkg::res_data]
  pub type QuoteFreightRes<'any> = SuperFreteResponse<
    &'any str,
    Vector<QuoteFreightResGeneric<Vector<QuoteFreightResPackage>, &'any str>>,
  >;
}
