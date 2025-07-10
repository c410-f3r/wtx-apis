#[wtx::pkg(data_format(json), id(crate::calendar::nager_date::NagerDateId), transport(http))]
pub(crate) mod pkg {
  use crate::calendar::nager_date::NagerDateHttpPkgsAux;
  use alloc::boxed::Box;
  use wtx::{
    client_api_framework::network::{HttpParams, transport::TransportParams},
    collection::Vector,
  };

  #[pkg::aux]
  impl<DRSR> NagerDateHttpPkgsAux<DRSR> {}

  #[pkg::before_sending]
  async fn before_sending(
    params: &mut V3CountryInfoParams<'_>,
    trans_params: &mut HttpParams,
  ) -> crate::Result<()> {
    trans_params
      .ext_req_params_mut()
      .uri
      .push_path(format_args!("/api/v3/CountryInfo/{}", params.country))?;
    Ok(())
  }

  #[derive(Debug)]
  #[pkg::params]
  pub struct V3CountryInfoParams<'any> {
    country: &'any str,
  }

  #[derive(Debug, serde::Serialize)]
  #[pkg::req_data]
  pub struct V3CountryInfoReq;

  #[pkg::res_data]
  pub type V3CountryInfoRes<'any> = Box<V3CountryInfo<&'any str>>;

  #[derive(Debug, serde::Deserialize)]
  #[serde(rename_all = "camelCase")]
  #[doc = _generic_res_data_elem_doc!()]
  pub struct V3CountryInfo<T> {
    /// For example, Spain.
    pub common_name: T,
    /// For example, Kingdom of Spain.
    pub official_name: T,
    /// ISO 3166-1 alpha-2.
    pub country_code: T,
    /// Continent.
    pub region: T,
    /// Adjacent countries.
    pub borders: Option<Vector<V3CountryInfo<T>>>,
  }
}
