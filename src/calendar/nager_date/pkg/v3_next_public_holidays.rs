#[wtx_macros::pkg(api(crate::calendar::nager_date::NagerDate), data_format(json), transport(http))]
pub(crate) mod pkg {
  use crate::calendar::nager_date::{NagerDateHttpPkgsAux, V3PublicHoliday};
  use alloc::vec::Vec;
  use wtx::client_api_framework::network::HttpReqParams;

  #[pkg::aux]
  impl<DRSR> NagerDateHttpPkgsAux<DRSR> {}

  #[pkg::before_sending]
  async fn before_sending(
    params: &mut V3NextPublicHolidaysParams<'_>,
    req_params: &mut HttpReqParams,
  ) -> crate::Result<()> {
    req_params.uri.push_path(format_args!("/api/v3/NextPublicHolidays/{}", params.country_code))?;
    Ok(())
  }

  #[derive(Debug)]
  #[pkg::params]
  pub struct V3NextPublicHolidaysParams<'any> {
    country_code: &'any str,
  }

  #[derive(Debug, serde::Serialize)]
  #[pkg::req_data]
  pub struct V3NextPublicHolidaysReq;

  #[pkg::res_data]
  pub type V3NextPublicHolidaysRes = Vec<V3PublicHoliday>;
}
