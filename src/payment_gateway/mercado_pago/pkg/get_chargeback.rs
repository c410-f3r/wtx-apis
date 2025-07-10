#[wtx::pkg(
  data_format(json),
  id(crate::payment_gateway::mercado_pago::MercadoPagoId),
  transport(http)
)]
pub(crate) mod pkg {
  use crate::payment_gateway::mercado_pago::{
    Chargeback, HttpPkgsAux, MercadoPago, MercadoPagoResponse, misc::manage_before_sending,
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
    params: &mut &str,
    trans: &mut __TRANSPORT,
    trans_params: &mut HttpParams,
  ) -> crate::Result<()>
  where
    for<'trans> __TRANSPORT: SendingReceivingTransport<&'trans mut HttpParams>,
    __DRSR: LeaseMut<SerdeJson>,
  {
    manage_before_sending((api, drsr.lease_mut(), trans, trans_params), bytes).await?;
    trans_params.ext_req_params_mut().uri.push_path(format_args!("/v1/chargebacks/{params}"))?;
    Ok(())
  }

  #[pkg::params]
  pub type GetChargebackParams<'any> = &'any str;

  #[derive(Debug, serde::Serialize)]
  #[pkg::req_data]
  pub struct GetChargebackReq;

  #[pkg::res_data]
  pub type GetChargebackRes<'de> = MercadoPagoResponse<&'de str, Chargeback<&'de str>>;
}
