#[wtx::pkg(data_format(json), id(crate::exchange::hyperliquid::HyperliquidId), transport(http))]
pub(crate) mod pkg {
  use crate::exchange::hyperliquid::{
    ExchangePayload, ExchangeResponse, HttpPkgsAux, order::BulkOrder,
  };

  #[pkg::aux]
  impl<A, DRSR> HttpPkgsAux<A, DRSR> {}

  #[pkg::req_data]
  pub type PlaceOrderReq<'any> = ExchangePayload<BulkOrder<'any>>;

  #[pkg::res_data]
  pub type PlaceOrderRes = ExchangeResponse<2>;
}
