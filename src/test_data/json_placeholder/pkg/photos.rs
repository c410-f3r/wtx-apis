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
    params.manage("photos", trans_params)?;
    Ok(())
  }

  #[pkg::params]
  pub type PhotosGenericParams<'any> = GenericParams<'any>;

  #[derive(Debug, serde::Serialize)]
  #[pkg::req_data]
  pub struct PhotosReq;

  #[pkg::res_data]
  pub type PhotosRes<'any> = GenericRes<&'any str>;

  /// Photo
  #[derive(Debug, serde::Deserialize)]
  #[serde(rename_all = "camelCase")]
  pub struct Photo<T> {
    /// Album id.
    pub album_id: u32,
    /// Id.
    pub id: u32,
    /// Title
    pub title: T,
    /// URL
    pub url: T,
    /// Thumbnail URL
    pub thumbnail_url: T,
  }
}
