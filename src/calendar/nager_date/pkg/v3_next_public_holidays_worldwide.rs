#[wtx_macros::pkg(data_format(json), id(crate::calendar::nager_date::NagerDateId), transport(http))]
pub(crate) mod pkg {
  use crate::calendar::nager_date::{NagerDateHttpPkgsAux, V3PublicHoliday};
  use wtx::{
    client_api_framework::network::{HttpParams, transport::TransportParams},
    collection::Vector,
  };

  #[pkg::aux]
  impl<DRSR> NagerDateHttpPkgsAux<DRSR> {}

  #[pkg::before_sending]
  async fn before_sending(trans_params: &mut HttpParams) -> crate::Result<()> {
    trans_params
      .ext_req_params_mut()
      .uri
      .push_path(format_args!("/api/v3/NextPublicHolidaysWorldwide"))?;
    Ok(())
  }

  #[derive(Debug, serde::Serialize)]
  #[pkg::req_data]
  pub struct V3NextPublicHolidaysWorldwideReq;

  #[pkg::res_data]
  pub type V3NextPublicHolidaysWorldwideRes = Vector<V3PublicHoliday>;
}
