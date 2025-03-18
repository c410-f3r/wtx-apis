#[wtx_macros::pkg(
  data_format(json),
  id(crate::test_data::json_placeholder::JsonPlaceholderId),
  transport(http)
)]
pub(crate) mod pkg {
  use crate::test_data::json_placeholder::{GenericParams, GenericRes, JsonPlaceholderHttpPkgsAux};
  use wtx::client_api_framework::network::HttpParams;

  #[pkg::aux]
  impl<DRSR> JsonPlaceholderHttpPkgsAux<DRSR> {}

  #[pkg::before_sending]
  async fn before_sending(
    params: &mut GenericParams<'_>,
    trans_params: &mut HttpParams,
  ) -> crate::Result<()> {
    params.manage("albums", trans_params)?;
    Ok(())
  }

  #[pkg::params]
  pub type AlbumsGenericParams<'any> = GenericParams<'any>;

  #[derive(Debug, serde::Serialize)]
  #[pkg::req_data]
  pub struct AlbumsReq;

  #[pkg::res_data]
  pub type AlbumsRes<'any> = GenericRes<&'any str>;

  /// Album
  #[derive(Debug, serde::Deserialize)]
  #[serde(rename_all = "camelCase")]
  pub struct Album<T> {
    /// User id.
    pub user_id: u32,
    /// Id
    pub id: u32,
    /// Title
    pub title: T,
  }
}
