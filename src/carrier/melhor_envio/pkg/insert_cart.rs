#[wtx_macros::pkg(
  data_format(json),
  id(crate::carrier::melhor_envio::MelhorEnvioId),
  transport(http)
)]
pub(crate) mod pkg {
  use crate::carrier::melhor_envio::{
    HttpPkgsAux, MelhorEnvio, MelhorEnvioResult, insert_cart_request::InsertCartRequest,
    insert_cart_response::InsertCartResponse, misc::manage_before_sending,
  };
  use wtx::client_api_framework::network::HttpParams;

  #[pkg::aux]
  impl<A, DRSR> HttpPkgsAux<A, DRSR> {}

  #[pkg::before_sending]
  async fn before_sending(
    api: &mut MelhorEnvio,
    trans_params: &mut HttpParams,
  ) -> crate::Result<()> {
    manage_before_sending(api, format_args!("/api/v2/me/cart"), trans_params).await
  }

  #[pkg::req_data]
  pub type InsertCartReq<'any> = InsertCartRequest<&'any str>;

  #[pkg::res_data]
  pub type InsertCartRes<'any> = MelhorEnvioResult<&'any str, InsertCartResponse<&'any str>>;
}
