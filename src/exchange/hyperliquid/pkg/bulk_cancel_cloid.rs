#[wtx::pkg(data_format(json), id(crate::exchange::hyperliquid::HyperliquidId), transport(http))]
mod pkg {
  use crate::exchange::hyperliquid::{
    ExchangeResponse, Hyperliquid, WsPkgsAux, action::Action, misc::manage_l1_action,
  };
  use wtx::misc::Lease;

  #[pkg::aux]
  impl<A, DRSR> WsPkgsAux<A, DRSR>
  where
    A: Lease<Hyperliquid>,
  {
    #[pkg::aux_data]
    fn bulk_cancel_cloid_data(
      &mut self,
      action: crate::exchange::hyperliquid::BulkCancelCloid<'_>,
      wallet: &k256::ecdsa::SigningKey,
    ) -> crate::Result<BulkCancelCloidReq> {
      manage_l1_action(Action::CancelByCloid(action), self, wallet, |_, el| el)?;
      Ok(BulkCancelCloidReq)
    }
  }

  #[pkg::req_data]
  #[derive(Debug, serde::Serialize)]
  pub struct BulkCancelCloidReq;

  #[pkg::res_data]
  pub type BulkCancelCloidRes = ExchangeResponse<2>;
}
