#[wtx_macros::pkg(
  data_format(json),
  id(crate::series::rick_and_morty::RickAndMortyId),
  transport(http)
)]
pub(crate) mod pkg {
  use crate::{
    misc::SliceByCommas,
    series::rick_and_morty::{CHARACTER_FRAGMENT, Location, RickAndMortyHttpPkgsAux},
  };
  use alloc::string::String;
  use core::fmt::Write;
  use wtx::{
    client_api_framework::network::transport::TransportParams,
    data_transformation::format::{GraphQlRequest, GraphQlResponse},
    http::Method,
    misc::Vector,
  };

  #[pkg::aux]
  impl<DRSR> RickAndMortyHttpPkgsAux<DRSR> {
    #[pkg::aux_data]
    fn locations_by_ids_data<'any>(
      &mut self,
      buffer: &'any mut String,
      ids: &[u32],
    ) -> crate::Result<LocationsByIdsReq<'any>> {
      buffer.clear();
      buffer
        .write_fmt(format_args!(
          r#"
            {CHARACTER_FRAGMENT}
            query {{
              locationsByIds(ids: ["{}"]) {{
                created
                dimension
                id
                name
                residents {{
                  ...CharacterFrag
                }}
                type
              }}
            }}
          "#,
          SliceByCommas(ids)
        ))
        .map_err(wtx::Error::from)?;
      self.tp.ext_req_params_mut().method = Method::Post;
      Ok(LocationsByIdsReq { operation_name: None, query: buffer, variables: None })
    }
  }

  #[pkg::req_data]
  pub type LocationsByIdsReq<'any> = GraphQlRequest<(), &'any str, ()>;

  #[pkg::res_data]
  pub type LocationsByIdsRes<'any> =
    GraphQlResponse<LocationsByIdsData<&'any str>, serde::de::IgnoredAny>;

  #[derive(Debug, serde::Deserialize)]
  #[doc = generic_data_doc!()]
  #[serde(rename_all = "camelCase")]
  pub struct LocationsByIdsData<T> {
    /// Locations by ids
    pub locations_by_ids: Vector<Location<T>>,
  }
}
