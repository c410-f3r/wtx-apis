#[wtx_macros::pkg(api(crate::blockchain::aptos::Aptos), data_format(verbatim), transport(http))]
pub(crate) mod pkg {
  use crate::blockchain::aptos::HttpPkgsAux;
  use serde::de::IgnoredAny;
  use wtx::client_api_framework::network::{HttpReqParams, HttpResParams, StatusCode};

  #[pkg::aux]
  impl<A, DRSR> HttpPkgsAux<A, DRSR> {}

  #[pkg::after_sending]
  async fn after_sending(
    api: &mut crate::blockchain::aptos::Aptos,
    res_params: &mut HttpResParams,
  ) -> crate::Result<()> {
    if res_params.status_code == StatusCode::Ok {
      api.fhrh.eval(res_params)?;
      Ok(())
    } else {
      Err(crate::Error::IncompatibleStatusCode(StatusCode::Ok, res_params.status_code))
    }
  }

  #[pkg::before_sending]
  async fn before_sending(
    params: &mut CheckBasicNodeHealthParams,
    req_params: &mut HttpReqParams,
  ) -> crate::Result<()> {
    req_params.headers.push_str("accept", "application/json, application/x-bcs")?;
    req_params.uri.push_path(format_args!("/-/healthy"))?;
    let _ = req_params.uri.query_writer()?.write_opt("duration_secs", params.duration_secs)?;
    Ok(())
  }

  #[derive(Debug)]
  #[pkg::params]
  pub struct CheckBasicNodeHealthParams {
    duration_secs: Option<u32>,
  }

  #[derive(Debug, serde::Serialize)]
  #[pkg::req_data]
  pub struct CheckBasicNodeHealthReq;

  #[pkg::res_data]
  pub type CheckBasicNodeHealthRes = IgnoredAny;
}
