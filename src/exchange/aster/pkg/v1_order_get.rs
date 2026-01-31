#[wtx::pkg(data_format(json), id(crate::exchange::aster::AsterId), transport(http))]
pub(crate) mod pkg {
  use crate::exchange::aster::{Aster, HttpPkgsAux, V1OrderGetReqParams, V1OrderGetResParams};
  use serde::Serialize;
  use wtx::{
    de::{AsciiSet, Encode, FormUrlSerializer, format::De, protocol::VerbatimEncoder},
    misc::LeaseMut,
  };

  #[pkg::aux]
  impl<A, DRSR> HttpPkgsAux<A, DRSR>
  where
    A: LeaseMut<Aster>,
    for<'any> VerbatimEncoder<&'any V1OrderGetReqParams>: Encode<De<DRSR>>,
  {
    #[pkg::aux_data]
    fn v1_order_get_data(&mut self, params: &V1OrderGetReqParams) -> crate::Result<()> {
      let wtx::client_api_framework::pkg::PkgsAux {
        api, bytes_buffer, send_bytes_buffer, tp, ..
      } = &mut self.0;
      params.serialize(FormUrlSerializer::new(AsciiSet::NON_ALPHANUMERIC, bytes_buffer))?;
      api.lease_mut().auth_req::<false>(
        bytes_buffer,
        format_args!("/api/v1/order"),
        send_bytes_buffer,
        tp,
      )?;
      Ok(())
    }
  }

  #[pkg::req_data]
  pub type V1OrderGetReq = ();

  #[pkg::res_data]
  pub type V1OrderGetRes = V1OrderGetResParams;
}
