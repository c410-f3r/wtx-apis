#[wtx::pkg(
  data_format(json),
  id(crate::payment_gateway::mercado_pago::MercadoPagoId),
  transport(http)
)]
pub(crate) mod pkg {
  use crate::payment_gateway::mercado_pago::{
    HttpPkgsAux, MercadoPago, MercadoPagoResponse, Refund, misc::manage_before_sending,
  };
  use wtx::{
    client_api_framework::network::{
      HttpParams,
      transport::{SendingReceivingTransport, TransportParams},
    },
    collection::Vector,
    de::format::SerdeJson,
    misc::LeaseMut,
  };

  #[pkg::aux]
  impl<A, DRSR> HttpPkgsAux<A, DRSR> {}

  #[pkg::before_sending]
  async fn before_sending<__DRSR, __TRANSPORT>(
    api: &mut MercadoPago,
    bytes: &mut Vector<u8>,
    drsr: &mut __DRSR,
    params: &mut GetRefundParams<'_>,
    trans: &mut __TRANSPORT,
    trans_params: &mut HttpParams,
  ) -> crate::Result<()>
  where
    for<'trans> __TRANSPORT: SendingReceivingTransport<&'trans mut HttpParams>,
    __DRSR: LeaseMut<SerdeJson>,
  {
    manage_before_sending((api, drsr.lease_mut(), trans, trans_params), bytes).await?;
    trans_params
      .ext_req_params_mut()
      .rrb
      .uri
      .push_path(format_args!("/v1/payments/{}/refunds/{}", params.id, params.refund_id))?;
    Ok(())
  }

  #[derive(Debug)]
  #[pkg::params]
  pub struct GetRefundParams<'any> {
    /// ID
    pub id: &'any str,
    /// Refund ID
    pub refund_id: &'any str,
  }

  #[derive(Debug, serde::Serialize)]
  #[pkg::req_data]
  pub struct GetRefundReq;

  #[pkg::res_data]
  pub type GetRefundRes<'de> = MercadoPagoResponse<&'de str, Refund<&'de str>>;
}
