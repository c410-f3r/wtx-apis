#[wtx_macros::pkg(
  api(crate::series::rick_and_morty::RickAndMorty),
  data_format(json),
  transport(http)
)]
pub(crate) mod pkg {
  use crate::series::rick_and_morty::{
    Location, Pagination, RickAndMortyHttpPkgsAux, CHARACTER_FRAGMENT,
  };
  use alloc::{string::String, vec::Vec};
  use core::fmt::Write;
  use wtx::{
    client_api_framework::network::transport::TransportParams,
    data_transformation::format::{GraphQlRequest, GraphQlResponse},
    http::Method,
  };

  #[pkg::aux]
  impl<DRSR> RickAndMortyHttpPkgsAux<DRSR> {
    #[pkg::aux_data]
    fn locations_data<'any>(
      &mut self,
      buffer: &'any mut String,
      dimension: &str,
      name: &str,
      page: u32,
      ty: &str,
    ) -> crate::Result<LocationsReq<'any>> {
      buffer.clear();
      buffer
        .write_fmt(format_args!(
          r#"
            {CHARACTER_FRAGMENT}
            query {{
              locations(
                filter: {{
                  dimension: "{dimension}",
                  name: "{name}",
                  type: "{ty}",
                }}
                page: {page},
              ) {{
                info {{
                  prev
                  pages
                  next
                  count
                }}
                results {{
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
            }}
          "#
        ))
        .map_err(wtx::Error::from)?;
      self.tp.ext_req_params_mut().method = Method::Post;
      Ok(LocationsReq { operation_name: None, query: buffer, variables: None })
    }
  }

  #[pkg::req_data]
  pub type LocationsReq<'any> = GraphQlRequest<(), &'any str, ()>;

  #[pkg::res_data]
  pub type LocationsRes = GraphQlResponse<LocationsData, serde::de::IgnoredAny>;

  #[derive(Debug, serde::Deserialize)]
  #[doc = generic_data_doc!()]
  pub struct Locations {
    /// Pagination
    pub info: Pagination,
    /// Locations
    pub results: Vec<Location>,
  }

  #[derive(Debug, serde::Deserialize)]
  #[doc = generic_data_doc!()]
  pub struct LocationsData {
    /// Locations
    pub locations: Locations,
  }
}
