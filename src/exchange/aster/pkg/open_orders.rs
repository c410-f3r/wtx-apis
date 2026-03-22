#[wtx::pkg(data_format(json), id(crate::exchange::aster::AsterId), transport(http))]
pub(crate) mod pkg {
  use crate::exchange::aster::{Aster, HttpPkgsAux, MarketReqParams, OrderResParams};
  use wtx::{client_api_framework::pkg::PkgsAux, collection::Vector, misc::LeaseMut};

  #[pkg::aux]
  impl<A, DRSR> HttpPkgsAux<A, DRSR>
  where
    A: LeaseMut<Aster>,
  {
    #[pkg::aux_data]
    fn open_orders_data(&mut self, params: &MarketReqParams<'_>) -> crate::Result<()> {
      let PkgsAux { api, bytes_buffer, encode_data, tp, .. } = &mut self.0;
      api.lease().auth_req::<false, _>(
        bytes_buffer,
        encode_data,
        Some(params),
        if api.lease().is_dex {
          format_args!("/api/v3/openOrders")
        } else {
          format_args!("/api/v1/openOrders")
        },
        None,
        tp,
      )
    }
  }

  #[pkg::req_data]
  pub type OpenOrdersReq = ();

  #[pkg::res_data]
  pub type OpenOrdersRes = Vector<OrderResParams>;
}
