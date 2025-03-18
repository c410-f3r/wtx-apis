#[wtx_macros::pkg(
  data_format(json),
  id(crate::payment_gateway::mercado_pago::MercadoPagoId),
  transport(http)
)]
pub(crate) mod pkg {
  use crate::payment_gateway::mercado_pago::{
    HttpPkgsAux, MercadoPago, MercadoPagoResponse, Preference, PreferenceResponse,
    misc::manage_before_sending,
  };
  use wtx::{
    client_api_framework::network::{
      HttpParams,
      transport::{SendingReceivingTransport, TransportParams},
    },
    data_transformation::dnsn::SerdeJson,
    http::Method,
    misc::{LeaseMut, Vector},
  };

  #[pkg::aux]
  impl<A, DRSR> HttpPkgsAux<A, DRSR> {}

  #[pkg::before_sending]
  async fn before_sending<__DRSR, __TRANSPORT>(
    api: &mut MercadoPago,
    bytes: &mut Vector<u8>,
    drsr: &mut __DRSR,
    trans: &mut __TRANSPORT,
    trans_params: &mut HttpParams,
  ) -> crate::Result<()>
  where
    for<'trans> __TRANSPORT: SendingReceivingTransport<&'trans mut HttpParams>,
    __DRSR: LeaseMut<SerdeJson>,
  {
    manage_before_sending((api, drsr.lease_mut(), trans, trans_params), bytes).await?;
    trans_params.ext_req_params_mut().method = Method::Post;
    trans_params.ext_req_params_mut().uri.push_path(format_args!("/checkout/preferences"))?;
    Ok(())
  }

  #[pkg::req_data]
  pub type PostPreferenceReq<T> = Preference<T>;

  #[pkg::res_data]
  pub type PostPreferenceRes<'de> = MercadoPagoResponse<&'de str, PreferenceResponse<&'de str>>;
}
