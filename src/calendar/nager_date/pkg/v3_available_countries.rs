#[wtx_macros::pkg(data_format(json), id(crate::calendar::nager_date::NagerDateId), transport(http))]
pub(crate) mod pkg {
  use crate::calendar::nager_date::NagerDateHttpPkgsAux;
  use wtx::{
    client_api_framework::network::{HttpParams, transport::TransportParams},
    collection::Vector,
  };

  #[pkg::aux]
  impl<DRSR> NagerDateHttpPkgsAux<DRSR> {}

  #[pkg::before_sending]
  async fn before_sending(trans_params: &mut HttpParams) -> crate::Result<()> {
    trans_params.ext_req_params_mut().uri.push_path(format_args!("/api/v3/AvailableCountries"))?;
    Ok(())
  }

  #[derive(Debug, serde::Serialize)]
  #[pkg::req_data]
  pub struct V3AvailableCountriesReq;

  #[pkg::res_data]
  pub type V3AvailableCountriesRes<'any> = Vector<V3AvailableCountries<&'any str>>;

  #[derive(Debug, serde::Deserialize)]
  #[serde(rename_all = "camelCase")]
  #[doc = _generic_res_data_elem_doc!()]
  pub struct V3AvailableCountries<T> {
    /// ISO 3166-1 alpha-2.
    pub country_code: T,
    /// English name
    pub name: T,
  }
}
