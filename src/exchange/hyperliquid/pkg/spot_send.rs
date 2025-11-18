#[wtx::pkg(data_format(json), id(crate::exchange::hyperliquid::HyperliquidId), transport(http))]
mod pkg {
  use crate::exchange::hyperliquid::{
    Chain, ExchangeResponse, HttpPkgsAux, Hyperliquid, SIGNATURE_CHAIN_ID,
    misc::{manage_typed_data, next_nonce},
  };
  use rust_decimal::Decimal;
  use wtx::misc::Lease;

  #[pkg::aux]
  impl<A, DRSR> HttpPkgsAux<A, DRSR>
  where
    A: Lease<Hyperliquid>,
  {
    #[pkg::aux_data]
    fn spot_send_data(
      &mut self,
      amount: Decimal,
      destination: &str,
      token: &str,
      wallet: &k256::ecdsa::SigningKey,
    ) -> crate::Result<SpotSendReq> {
      let nonce = next_nonce()?;
      let action = crate::exchange::hyperliquid::SpotSend {
        signature_chain_id: SIGNATURE_CHAIN_ID,
        hyperliquid_chain: Chain::from_api(self.api.lease()),
        time: nonce,
        destination,
        token,
        amount,
      };
      manage_typed_data(nonce, self, action, wallet)?;
      Ok(SpotSendReq)
    }
  }

  #[pkg::req_data]
  #[derive(Debug, serde::Serialize)]
  pub struct SpotSendReq;

  #[pkg::res_data]
  pub type SpotSendRes = ExchangeResponse<2>;
}
