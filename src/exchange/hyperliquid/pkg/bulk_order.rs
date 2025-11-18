#[wtx::pkg(data_format(json), id(crate::exchange::hyperliquid::HyperliquidId), transport(ws))]
mod pkg {
  use crate::exchange::hyperliquid::{
    ExchangeResponse, Hyperliquid, WebSocketReq, WebSocketReqParams, WebSocketRes, WebSocketTy,
    WsPkgsAux, action::Action, misc::manage_l1_action,
  };
  use wtx::{http::Method, misc::Lease};

  #[pkg::aux]
  impl<A, DRSR> WsPkgsAux<A, DRSR>
  where
    A: Lease<Hyperliquid>,
  {
    #[pkg::aux_data]
    fn bulk_order_data(
      &mut self,
      action: crate::exchange::hyperliquid::BulkOrder<'_>,
      wallet: &k256::ecdsa::SigningKey,
    ) -> crate::Result<BulkOrderReq> {
      manage_l1_action(Action::Order(action), self, wallet, |id, el| WebSocketReq {
        id,
        method: Method::Post,
        request: WebSocketReqParams { ty: WebSocketTy::Action, payload: el },
      })?;
      Ok(BulkOrderReq)
    }
  }

  #[pkg::req_data]
  #[derive(Debug, serde::Serialize)]
  pub struct BulkOrderReq;

  #[pkg::res_data]
  pub type BulkOrderRes = WebSocketRes<ExchangeResponse<2>>;
}
