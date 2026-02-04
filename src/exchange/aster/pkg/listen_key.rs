#[wtx::pkg(data_format(json), id(crate::exchange::aster::AsterId), transport(http))]
pub(crate) mod pkg {
  use crate::exchange::aster::{Aster, CexSignParams, HttpPkgsAux};
  use wtx::{client_api_framework::pkg::PkgsAux, collection::ArrayStringU8, misc::LeaseMut};

  #[pkg::aux]
  impl<A, DRSR> HttpPkgsAux<A, DRSR>
  where
    A: LeaseMut<Aster>,
  {
    #[pkg::aux_data]
    fn listen_key_data(&mut self, params: Option<&CexSignParams>) -> crate::Result<()> {
      let PkgsAux { api, bytes_buffer, send_bytes_buffer, tp, .. } = &mut self.0;
      api.lease().auth_req::<true, _>(
        bytes_buffer,
        params,
        if api.lease().is_dex {
          format_args!("/api/v3/listenKey")
        } else {
          format_args!("/api/v1/listenKey")
        },
        send_bytes_buffer,
        None,
        tp,
      )
    }
  }

  #[pkg::req_data]
  pub type ListenKeyReq = ();

  #[pkg::res_data]
  #[derive(Debug, serde::Deserialize)]
  #[serde(rename_all = "camelCase")]
  pub struct ListenKeyRes {
    /// Listen Key
    pub listen_key: ArrayStringU8<64>,
  }
}
