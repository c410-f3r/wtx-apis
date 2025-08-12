#[wtx::pkg(data_format(json), id(crate::calendar::nager_date::NagerDateId), transport(http))]
pub(crate) mod pkg {
  use crate::calendar::nager_date::NagerDateHttpPkgsAux;
  use wtx::{
    client_api_framework::network::{HttpParams, transport::TransportParams},
    http::StatusCode,
  };

  #[pkg::aux]
  impl<DRSR> NagerDateHttpPkgsAux<DRSR> {}

  #[pkg::after_sending]
  async fn after_sending(trans_params: &mut HttpParams) -> crate::Result<()> {
    if trans_params.ext_res_params_mut().status_code == StatusCode::Ok {
      Ok(())
    } else {
      Err(crate::Error::IncompatibleStatusCode(
        StatusCode::Ok,
        trans_params.ext_res_params_mut().status_code,
      ))
    }
  }

  #[pkg::before_sending]
  async fn before_sending(
    params: &mut V3IsTodayPublicHolidayParams<'_>,
    trans_params: &mut HttpParams,
  ) -> crate::Result<()> {
    trans_params
      .ext_req_params_mut()
      .uri
      .push_path(format_args!("/api/v3/IsTodayPublicHoliday/{}", params.country_code))?;
    let _ = trans_params
      .ext_req_params_mut()
      .uri
      .query_writer()?
      .write_opt("countyCode", params.county_code)?
      .write_opt("offset", params.offset)?;
    Ok(())
  }

  #[derive(Debug)]
  #[pkg::params]
  pub struct V3IsTodayPublicHolidayParams<'any> {
    country_code: &'any str,
    county_code: Option<&'any str>,
    offset: Option<i8>,
  }

  #[derive(Debug, serde::Serialize)]
  #[pkg::req_data]
  pub struct V3IsTodayPublicHolidayReq;

  #[pkg::res_data]
  pub type V3IsTodayPublicHolidayRes = ();
}
