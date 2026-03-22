#[wtx::pkg(data_format(verbatim), id(crate::exchange::aster::AsterId), transport(http))]
pub(crate) mod pkg {
  use crate::exchange::aster::{Aster, HttpPkgsAux, OrderReqParams, OrderResParams};
  use wtx::{
    client_api_framework::{network::transport::TransportParams, pkg::PkgsAux},
    http::Method,
    misc::LeaseMut,
  };

  #[pkg::aux]
  impl<A, DRSR> HttpPkgsAux<A, DRSR>
  where
    A: LeaseMut<Aster>,
  {
    #[pkg::aux_data]
    fn order_delete_data(&mut self, params: &OrderReqParams<'_>) -> crate::Result<()> {
      let PkgsAux { api, bytes_buffer, encode_data, tp, .. } = &mut self.0;
      api.lease().auth_req::<false, _>(
        bytes_buffer,
        encode_data,
        Some(params),
        if api.lease().is_dex {
          format_args!("/api/v3/order")
        } else {
          format_args!("/api/v1/order")
        },
        None,
        tp,
      )?;
      self.tp.ext_req_params_mut().method = Method::Delete;
      Ok(())
    }
  }

  #[pkg::req_data]
  pub type OrderDeleteReq = ();

  #[pkg::res_data]
  pub type OrderDeleteRes = OrderResParams;
}
