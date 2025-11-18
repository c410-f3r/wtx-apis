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
    fn bulk_modify_data(
      &mut self,
      action: crate::exchange::hyperliquid::BulkModify<'_>,
      wallet: &k256::ecdsa::SigningKey,
    ) -> crate::Result<BulkModifyReq> {
      manage_l1_action(Action::BulkModify(action), self, wallet, |_, el| el)?;
      Ok(BulkModifyReq)
    }
  }

  #[pkg::req_data]
  #[derive(Debug, serde::Serialize)]
  pub struct BulkModifyReq;

  #[pkg::res_data]
  pub type BulkModifyRes = ExchangeResponse<2>;
}
