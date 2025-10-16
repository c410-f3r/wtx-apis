#[wtx::pkg(
  data_format(json),
  id(crate::test_data::json_placeholder::JsonPlaceholderId),
  transport(http)
)]
pub(crate) mod pkg {
  use crate::test_data::json_placeholder::{GenericParams, GenericRes, JsonPlaceholderHttpPkgsAux};
  use alloc::string::String;
  use wtx::client_api_framework::network::HttpParams;

  #[pkg::aux]
  impl<DRSR> JsonPlaceholderHttpPkgsAux<DRSR> {}

  #[pkg::before_sending]
  async fn before_sending(
    params: &mut GenericParams<'_>,
    trans_params: &mut HttpParams,
  ) -> crate::Result<()> {
    params.manage("comments", trans_params)?;
    Ok(())
  }

  #[pkg::params]
  pub type CommentsGenericParams<'any> = GenericParams<'any>;

  #[derive(Debug, serde::Serialize)]
  #[pkg::req_data]
  pub struct CommentsReq;

  #[pkg::res_data]
  pub type CommentsRes<'any> = GenericRes<String>;

  /// Comment
  #[derive(Debug, serde::Deserialize)]
  #[serde(rename_all = "camelCase")]
  pub struct Comment<T> {
    /// Post id
    pub post_id: u32,
    /// Id
    pub id: u32,
    /// Name
    pub name: T,
    /// Email
    pub email: T,
    /// Body
    pub body: T,
  }
}
