#[wtx::pkg(data_format(verbatim), id(crate::exchange::aster::AsterId), transport(http))]
pub(crate) mod pkg {
  use crate::exchange::aster::{Aster, HttpPkgsAux, V1OrderPostReqParams, V1OrderPostResParams};
  use serde::Serialize;
  use wtx::{
    de::{AsciiSet, FormUrlSerializer},
    misc::LeaseMut,
  };

  #[pkg::aux]
  impl<A, DRSR> HttpPkgsAux<A, DRSR>
  where
    A: LeaseMut<Aster>,
  {
    #[pkg::aux_data]
    fn v1_order_post_data(&mut self, params: &V1OrderPostReqParams) -> crate::Result<()> {
      let wtx::client_api_framework::pkg::PkgsAux {
        api, bytes_buffer, send_bytes_buffer, tp, ..
      } = &mut self.0;
      params.serialize(FormUrlSerializer::new(AsciiSet::NON_ALPHANUMERIC, bytes_buffer))?;
      api.lease_mut().auth_req::<true>(
        bytes_buffer,
        format_args!("/api/v1/order"),
        send_bytes_buffer,
        tp,
      )?;
      Ok(())
    }
  }

  #[pkg::req_data]
  pub type V1OrderPostReq = ();

  #[pkg::res_data]
  pub type V1OrderPostRes = V1OrderPostResParams;
}
