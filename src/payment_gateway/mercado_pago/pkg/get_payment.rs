#[wtx_macros::pkg(
  data_format(json),
  id(crate::payment_gateway::mercado_pago::MercadoPagoId),
  transport(http)
)]
pub(crate) mod pkg {
  use crate::payment_gateway::mercado_pago::{
    HttpPkgsAux, MercadoPago, MercadoPagoResponse, Payment, misc::manage_before_sending,
  };
  use wtx::{
    client_api_framework::network::{
      HttpParams,
      transport::{SendingReceivingTransport, TransportParams},
    },
    collection::Vector,
    data_transformation::dnsn::SerdeJson,
    misc::LeaseMut,
  };

  #[pkg::aux]
  impl<A, DRSR> HttpPkgsAux<A, DRSR> {}

  #[pkg::before_sending]
  async fn before_sending<__DRSR, __TRANSPORT>(
    api: &mut MercadoPago,
    bytes: &mut Vector<u8>,
    drsr: &mut __DRSR,
    params: &mut u64,
    trans: &mut __TRANSPORT,
    trans_params: &mut HttpParams,
  ) -> crate::Result<()>
  where
    for<'trans> __TRANSPORT: SendingReceivingTransport<&'trans mut HttpParams>,
    __DRSR: LeaseMut<SerdeJson>,
  {
    manage_before_sending((api, drsr.lease_mut(), trans, trans_params), bytes).await?;
    trans_params.ext_req_params_mut().uri.push_path(format_args!("/v1/payments/{}", *params))?;
    Ok(())
  }

  #[pkg::params]
  pub type GetPaymentParams = u64;

  #[pkg::req_data]
  #[derive(Debug, serde::Serialize)]
  pub struct GetPaymentReq;

  #[pkg::res_data]
  pub type GetPaymentRes<'de> = MercadoPagoResponse<&'de str, Payment<&'de str>>;
}
