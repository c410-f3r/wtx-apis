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
    fn usd_send_data(
      &mut self,
      amount: Decimal,
      destination: &str,
      wallet: &k256::ecdsa::SigningKey,
    ) -> crate::Result<UsdSendReq> {
      let nonce = next_nonce()?;
      let action = crate::exchange::hyperliquid::usd_send::UsdSend {
        signature_chain_id: SIGNATURE_CHAIN_ID,
        hyperliquid_chain: Chain::from_api(self.api.lease()),
        destination,
        amount,
        time: nonce,
      };
      manage_typed_data(nonce, self, action, wallet)?;
      Ok(UsdSendReq)
    }
  }

  #[pkg::req_data]
  #[derive(Debug, serde::Serialize)]
  pub struct UsdSendReq;

  #[pkg::res_data]
  pub type UsdSendRes = ExchangeResponse<2>;
}
