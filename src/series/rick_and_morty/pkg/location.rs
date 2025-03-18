#[wtx_macros::pkg(
  data_format(json),
  id(crate::series::rick_and_morty::RickAndMortyId),
  transport(http)
)]
pub(crate) mod pkg {
  use crate::series::rick_and_morty::{CHARACTER_FRAGMENT, Location, RickAndMortyHttpPkgsAux};
  use alloc::string::String;
  use core::fmt::Write;
  use wtx::{
    client_api_framework::network::transport::TransportParams,
    data_transformation::format::{GraphQlRequest, GraphQlResponse},
    http::Method,
  };

  #[pkg::aux]
  impl<DRSR> RickAndMortyHttpPkgsAux<DRSR> {
    #[pkg::aux_data]
    fn location_data<'any>(
      &mut self,
      buffer: &'any mut String,
      id: u32,
    ) -> crate::Result<LocationReq<'any>> {
      buffer.clear();
      buffer
        .write_fmt(format_args!(
          r#"
            {CHARACTER_FRAGMENT}
            query {{
              location(id: "{id}") {{
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
          "#
        ))
        .map_err(wtx::Error::from)?;
      self.tp.ext_req_params_mut().method = Method::Post;
      Ok(LocationReq { operation_name: None, query: buffer, variables: None })
    }
  }

  #[pkg::req_data]
  pub type LocationReq<'any> = GraphQlRequest<(), &'any str, ()>;

  #[pkg::res_data]
  pub type LocationRes<'any> = GraphQlResponse<LocationData<&'any str>, serde::de::IgnoredAny>;

  #[derive(Debug, serde::Deserialize)]
  #[doc = generic_data_doc!()]
  pub struct LocationData<T> {
    /// Location
    pub location: Location<T>,
  }
}
