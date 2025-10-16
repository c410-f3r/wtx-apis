#[wtx::pkg(data_format(json), id(crate::secret_management::vault::VaultId), transport(http))]
pub(crate) mod pkg {
  use crate::secret_management::vault::{HttpPkgsAux, PairVector, Vault, VaultResponse};
  use alloc::string::String;
  use wtx::{
    client_api_framework::network::{HttpParams, transport::TransportParams},
    http::Header,
  };

  #[pkg::aux]
  impl<A, DRSR> HttpPkgsAux<A, DRSR> {}

  #[pkg::before_sending]
  async fn before_sending(
    api: &mut Vault,
    params: &mut Kv2ReadSecretVersionParams<'_>,
    trans_params: &mut HttpParams,
  ) -> crate::Result<()> {
    trans_params
      .ext_req_params_mut()
      .headers
      .push_from_iter(Header::from_name_and_value("x-vault-token", [api.token.as_str()]))?;
    trans_params
      .ext_req_params_mut()
      .uri
      .push_path(format_args!("/{}/data/{}", params.secret_mount_path, params.path))?;
    if let Some(elem) = params.version {
      let _ = trans_params.ext_req_params_mut().uri.query_writer("version", elem)?;
    }
    Ok(())
  }

  #[derive(Debug)]
  #[pkg::params]
  pub struct Kv2ReadSecretVersionParams<'any> {
    path: &'any str,
    secret_mount_path: &'any str,
    version: Option<u32>,
  }

  #[derive(Debug, serde::Serialize)]
  #[pkg::req_data]
  pub struct Kv2ReadSecretVersionReq;

  #[pkg::res_data]
  pub type Kv2ReadSecretVersionRes<'any> = VaultResponse<Kv2ReadSecretVersionResponse<&'any str>>;

  /// Response
  #[derive(Debug, serde::Deserialize)]
  pub struct Kv2ReadSecretVersionResponse<T> {
    /// Data
    pub data: PairVector<String, String>,
    /// Metadata
    pub metadata: SecretVersionMetadata<T>,
  }

  /// Secret version metadata
  #[derive(Debug, serde::Deserialize)]
  pub struct SecretVersionMetadata<T> {
    /// Created time
    pub created_time: T,
    /// Deletion time
    pub deletion_time: T,
    /// Destroyed
    pub destroyed: bool,
    /// Version
    pub version: u64,
  }
}
