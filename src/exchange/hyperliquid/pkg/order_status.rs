#[wtx::pkg(data_format(json), id(crate::exchange::hyperliquid::HyperliquidId), transport(ws))]
mod pkg {
  use crate::exchange::hyperliquid::{
    Hyperliquid, OrderInfo, WebSocketReq, WebSocketReqParams, WebSocketTy, WsPkgsAux,
    info_req::InfoReq,
  };
  use wtx::{http::Method, misc::Lease};

  #[pkg::aux]
  impl<A, DRSR> WsPkgsAux<A, DRSR>
  where
    A: Lease<Hyperliquid>,
  {
    #[pkg::aux_data]
    fn order_status_data(&mut self, oid: u64, user: &str) -> crate::Result<OrderStatusReq> {
      let id = self.built_requests;
      serde_json::to_writer(
        &mut self.bytes_buffer,
        &WebSocketReq {
          id,
          method: Method::Post,
          request: WebSocketReqParams {
            ty: WebSocketTy::Info,
            payload: InfoReq::OrderStatus { user, oid },
          },
        },
      )?;
      self.send_bytes_buffer = true;
      Ok(OrderStatusReq)
    }
  }

  #[pkg::req_data]
  #[derive(Debug, serde::Serialize)]
  pub struct OrderStatusReq;

  #[pkg::res_data]
  #[derive(Debug, serde::Deserialize)]
  pub struct OrderStatusRes {
    #[serde(default)]
    pub order: Option<OrderInfo>,
  }
}
