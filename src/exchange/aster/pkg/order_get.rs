#[wtx::pkg(data_format(json), id(crate::exchange::aster::AsterId), transport(http))]
pub(crate) mod pkg {
  use crate::exchange::aster::{Aster, HttpPkgsAux, OrderReqParams, OrderResParams};
  use wtx::{client_api_framework::pkg::PkgsAux, misc::LeaseMut};

  #[pkg::aux]
  impl<A, DRSR> HttpPkgsAux<A, DRSR>
  where
    A: LeaseMut<Aster>,
  {
    #[pkg::aux_data]
    fn order_get_data(&mut self, params: &OrderReqParams<'_>) -> crate::Result<()> {
      let PkgsAux { api, bytes_buffer, send_bytes_buffer, tp, .. } = &mut self.0;
      api.lease().auth_req::<false, _>(
        bytes_buffer,
        Some(params),
        if api.lease().is_dex {
          format_args!("/api/v3/order")
        } else {
          format_args!("/api/v1/order")
        },
        send_bytes_buffer,
        None,
        tp,
      )
    }
  }

  #[pkg::req_data]
  pub type OrderGetReq = ();

  #[pkg::res_data]
  pub type OrderGetRes = OrderResParams;
}
