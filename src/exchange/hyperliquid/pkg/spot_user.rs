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
    fn spot_user_data(
      &mut self,
      action: crate::exchange::hyperliquid::SpotUser,
      wallet: &k256::ecdsa::SigningKey,
    ) -> crate::Result<SpotUserReq> {
      manage_l1_action(Action::SpotUser(action), self, wallet, |_, el| el)?;
      Ok(SpotUserReq)
    }
  }

  #[pkg::req_data]
  #[derive(Debug, serde::Serialize)]
  pub struct SpotUserReq;

  #[pkg::res_data]
  pub type SpotUserRes = ExchangeResponse<2>;
}
