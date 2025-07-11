#[wtx::pkg(data_format(json), id(crate::carrier::super_frete::SuperFreteId), transport(http))]
pub(crate) mod pkg {
  use crate::carrier::super_frete::{
    HttpPkgsAux, SuperFrete, SuperFreteResponse, WebhookEvent,
    misc::manage_token,
    quote_freight_res::{QuoteFreightResGeneric, QuoteFreightResPackage},
  };
  use wtx::{
    calendar::{DateTime, DynTz},
    client_api_framework::network::HttpParams,
    collection::{ArrayVectorU8, Vector},
  };

  #[pkg::aux]
  impl<A, DRSR> HttpPkgsAux<A, DRSR> {}

  #[pkg::before_sending]
  async fn before_sending(
    api: &mut SuperFrete,
    trans_params: &mut HttpParams,
  ) -> crate::Result<()> {
    manage_token(api, "/api/v0/webhook", trans_params)
  }

  #[derive(Debug, serde::Serialize)]
  #[pkg::req_data]
  pub struct CreateWebhookReq<S> {
    /// See [`WebhookTy`].
    pub events: ArrayVectorU8<WebhookEvent, 6>,
    /// Identifier
    pub name: S,
    /// URL that will be receive notifications
    pub url: S,
  }

  #[pkg::res_data]
  pub type CreateWebhookRes<'any> = SuperFreteResponse<
    CreateWebhookResData<'any>,
    Vector<QuoteFreightResGeneric<Vector<QuoteFreightResPackage>, &'any str>>,
  >;

  /// Create WebHook Response Data
  #[derive(Debug, serde::Deserialize)]
  pub struct CreateWebhookResData<'str> {
    /// Created at
    pub created_at: DateTime<DynTz>,
    /// Id
    pub id: &'str str,
    /// Is active
    pub is_active: bool,
    /// Name
    pub name: &'str str,
    /// Secret token
    pub secret_token: &'str str,
    /// Url
    pub url: &'str str,
  }
}
