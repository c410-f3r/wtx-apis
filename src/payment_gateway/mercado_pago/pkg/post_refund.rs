#[wtx_macros::pkg(
  data_format(json),
  id(crate::payment_gateway::mercado_pago::MercadoPagoId),
  transport(http)
)]
pub(crate) mod pkg {
  use crate::payment_gateway::mercado_pago::{
    HttpPkgsAux, MercadoPago, MercadoPagoResponse, Refund, misc::manage_before_sending,
  };
  use rust_decimal::Decimal;
  use wtx::{
    client_api_framework::network::{
      HttpParams,
      transport::{SendingReceivingTransport, TransportParams},
    },
    collection::Vector,
    data_transformation::dnsn::SerdeJson,
    http::Header,
    misc::LeaseMut,
  };

  #[pkg::aux]
  impl<A, DRSR> HttpPkgsAux<A, DRSR> {}

  #[pkg::before_sending]
  async fn before_sending<__DRSR, __TRANSPORT>(
    api: &mut MercadoPago,
    bytes: &mut Vector<u8>,
    drsr: &mut __DRSR,
    params: &mut CreateChargebackParams<'_>,
    trans: &mut __TRANSPORT,
    trans_params: &mut HttpParams,
  ) -> crate::Result<()>
  where
    for<'trans> __TRANSPORT: SendingReceivingTransport<&'trans mut HttpParams>,
    __DRSR: LeaseMut<SerdeJson>,
  {
    manage_before_sending((api, drsr.lease_mut(), trans, trans_params), bytes).await?;
    trans_params.ext_req_params_mut().headers.push_from_iter(Header::from_name_and_value(
      "x-idempotency-key",
      [params.idempotency_key].into_iter(),
    ))?;
    trans_params
      .ext_req_params_mut()
      .uri
      .push_path(format_args!("/v1/chargebacks/{}", params.id))?;
    Ok(())
  }

  #[derive(Debug)]
  #[pkg::params]
  pub struct CreateChargebackParams<'any> {
    id: &'any str,
    idempotency_key: &'any str,
  }

  #[derive(Debug, serde::Serialize)]
  #[pkg::req_data]
  pub struct PostChargebackReq {
    /// Amount
    #[serde(with = "rust_decimal::serde::float_option")]
    pub amount: Option<Decimal>,
  }

  #[pkg::res_data]
  pub type PostChargebackRes<'de> = MercadoPagoResponse<&'de str, Refund<&'de str>>;
}
