#[wtx::pkg(data_format(json), id(crate::calendar::nager_date::NagerDateId), transport(http))]
pub(crate) mod pkg {
  use crate::calendar::nager_date::NagerDateHttpPkgsAux;
  use wtx::{
    client_api_framework::network::{HttpParams, transport::TransportParams},
    collection::Vector,
  };

  #[pkg::aux]
  impl<DRSR> NagerDateHttpPkgsAux<DRSR> {}

  #[pkg::before_sending]
  async fn before_sending(
    params: &mut V3LongWeekendParams<'_>,
    trans_params: &mut HttpParams,
  ) -> crate::Result<()> {
    trans_params
      .ext_req_params_mut()
      .uri
      .push_path(format_args!("/api/v3/LongWeekend/{}/{}", params.year, params.country_code))?;
    Ok(())
  }

  #[derive(Debug)]
  #[pkg::params]
  pub struct V3LongWeekendParams<'any> {
    year: i16,
    country_code: &'any str,
  }

  #[derive(Debug, serde::Serialize)]
  #[pkg::req_data]
  pub struct V3LongWeekendReq;

  #[pkg::res_data]
  pub type V3LongWeekendRes<'any> = Vector<V3LongWeekend<&'any str>>;

  #[derive(Debug, serde::Deserialize)]
  #[serde(rename_all = "camelCase")]
  #[doc = _generic_res_data_elem_doc!()]
  pub struct V3LongWeekend<T> {
    /// Start date.
    pub start_date: T,
    /// End date.
    pub end_date: T,
    /// Number os days.
    pub day_count: u8,
    /// A working day that is sandwiched between a holiday and a weekend.
    pub need_bridge_day: bool,
  }
}
