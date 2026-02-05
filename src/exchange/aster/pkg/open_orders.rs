use crate::exchange::aster::CexSignParams;

/// Structure sent when querying open orders
#[derive(Debug, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct OpenOrdersReqParams<'any> {
  /// Market pair
  pub symbol: &'any str,
  /// See [`SignParams`].
  #[serde(flatten)]
  pub sign_params: Option<CexSignParams>,
}

#[wtx::pkg(data_format(json), id(crate::exchange::aster::AsterId), transport(http))]
pub(crate) mod pkg {
  use crate::exchange::aster::{Aster, HttpPkgsAux, OpenOrdersReqParams, OrderResParams};
  use wtx::{client_api_framework::pkg::PkgsAux, collection::Vector, misc::LeaseMut};

  #[pkg::aux]
  impl<A, DRSR> HttpPkgsAux<A, DRSR>
  where
    A: LeaseMut<Aster>,
  {
    #[pkg::aux_data]
    fn open_orders_data(&mut self, params: &OpenOrdersReqParams<'_>) -> crate::Result<()> {
      let PkgsAux { api, bytes_buffer, send_bytes_buffer, tp, .. } = &mut self.0;
      api.lease().auth_req::<false, _>(
        bytes_buffer,
        Some(params),
        if api.lease().is_dex {
          format_args!("/api/v3/openOrders")
        } else {
          format_args!("/api/v1/openOrders")
        },
        send_bytes_buffer,
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
