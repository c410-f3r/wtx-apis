#[wtx_macros::pkg(api(crate::calendar::nager_date::NagerDate), data_format(json), transport(http))]
pub(crate) mod pkg {
  use crate::calendar::nager_date::NagerDateHttpPkgsAux;
  use alloc::vec::Vec;
  use wtx::{client_api_framework::network::HttpReqParams, misc::ArrayString};

  #[pkg::aux]
  impl<DRSR> NagerDateHttpPkgsAux<DRSR> {}

  #[pkg::before_sending]
  async fn before_sending(req_params: &mut HttpReqParams) -> crate::Result<()> {
    req_params.uri.push_path(format_args!("/api/v3/AvailableCountries"))?;
    Ok(())
  }

  #[derive(Debug, serde::Serialize)]
  #[pkg::req_data]
  pub struct V3AvailableCountriesReq;

  #[pkg::res_data]
  pub type V3AvailableCountriesRes = Vec<V3AvailableCountries>;

  #[derive(Debug, serde::Deserialize)]
  #[serde(rename_all = "camelCase")]
  #[doc = _generic_res_data_elem_doc!()]
  pub struct V3AvailableCountries {
    /// ISO 3166-1 alpha-2.
    pub country_code: ArrayString<2>,
    /// English name
    pub name: ArrayString<22>,
  }
}
