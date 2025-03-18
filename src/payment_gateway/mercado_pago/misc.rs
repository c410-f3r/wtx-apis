use crate::{
  misc::{_apply_auth_header, _manage_client_credentials, OauthResponse},
  payment_gateway::mercado_pago::{MercadoPago, OauthReq},
};
use wtx::{
  client_api_framework::network::{
    HttpParams,
    transport::{SendingReceivingTransport, TransportParams},
  },
  data_transformation::{
    dnsn::De,
    format::{VerbatimRequest, VerbatimResponse},
  },
  misc::{DecodeSeq, Encode, Vector},
};

pub(crate) async fn manage_before_sending<DRSR, T>(
  (api, drsr, trans, trans_params): (&mut MercadoPago, &mut DRSR, T, &mut HttpParams),
  buffer: &mut Vector<u8>,
) -> crate::Result<()>
where
  for<'any> VerbatimRequest<OauthReq<'any>>: Encode<De<&'any mut DRSR>>,
  for<'any> T: SendingReceivingTransport<&'any mut HttpParams>,
  for<'de> VerbatimResponse<OauthResponse<&'de str>>: DecodeSeq<'de, De<DRSR>>,
{
  trans_params.ext_req_params_mut().uri.push_path(format_args!("/oauth/token"))?;
  let is_test = api.is_test;
  _manage_client_credentials((api, drsr, trans, trans_params), buffer, |local_bytes| {
    let _ = local_bytes.extend_from_copyable_slices([
      "&test_token=".as_bytes(),
      if is_test { "true".as_bytes() } else { "false".as_bytes() },
    ])?;
    Ok(())
  })
  .await?;
  trans_params.ext_req_params_mut().uri.truncate_with_initial_len();
  _apply_auth_header(trans_params, &api.common.access_token)?;
  Ok(())
}
