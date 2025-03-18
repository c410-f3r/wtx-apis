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
    params.manage("users", trans_params)?;
    Ok(())
  }

  #[pkg::params]
  pub type UsersGenericParams<'any> = GenericParams<'any>;

  #[derive(Debug, serde::Serialize)]
  #[pkg::req_data]
  pub struct UsersReq;

  #[pkg::res_data]
  pub type UsersRes<'any> = GenericRes<&'any str>;

  /// User
  #[derive(Debug, serde::Deserialize)]
  #[serde(rename_all = "camelCase")]
  pub struct User<T> {
    /// Id
    pub id: u32,
    /// Name
    pub name: T,
    /// Username
    pub username: T,
    /// Email
    pub email: T,
    /// Address
    pub address: UserAddress<T>,
    /// Phone
    pub phone: T,
    /// Website
    pub website: T,
    /// Company
    pub company: UserCompany<T>,
  }

  /// User address
  #[derive(Debug, serde::Deserialize)]
  #[serde(rename_all = "camelCase")]
  pub struct UserAddress<T> {
    /// Street
    pub street: T,
    /// Suite
    pub suite: T,
    /// City
    pub city: T,
    /// Zip-code
    pub zipcode: T,
    /// User geographic parameters
    pub geo: UserGeoParams<T>,
  }

  /// User company
  #[derive(Debug, serde::Deserialize)]
  #[serde(rename_all = "camelCase")]
  pub struct UserCompany<T> {
    /// Name
    pub name: T,
    /// What the company does.
    pub catch_phrase: T,
    /// Tags
    pub bs: T,
  }

  /// User geographic parameters.
  #[derive(Debug, serde::Deserialize)]
  #[serde(rename_all = "camelCase")]
  pub struct UserGeoParams<T> {
    /// Latitude
    pub lat: T,
    /// Longitude
    pub lng: T,
  }
}
