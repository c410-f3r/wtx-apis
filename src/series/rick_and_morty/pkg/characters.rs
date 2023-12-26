#[wtx_macros::pkg(
  api(crate::series::rick_and_morty::RickAndMorty),
  data_format(json),
  transport(http)
)]
pub(crate) mod pkg {
  use crate::series::rick_and_morty::{
    Character, Pagination, RickAndMortyHttpPkgsAux, CHARACTER_FRAGMENT,
  };
  use alloc::{string::String, vec::Vec};
  use core::fmt::Write;
  use wtx::client_api_framework::{
    data_format::{GraphQlRequest, GraphQlResponse},
    network::{transport::TransportParams, HttpMethod},
  };

  #[pkg::aux]
  impl<DRSR> RickAndMortyHttpPkgsAux<DRSR> {
    #[pkg::aux_data]
    fn characters_data<'any>(
      &mut self,
      buffer: &'any mut String,
      gender: &str,
      name: &str,
      page: u32,
      species: &str,
      status: &str,
      ty: &str,
    ) -> crate::Result<CharactersReq<'any>> {
      buffer.clear();
      buffer
        .write_fmt(format_args!(
          r#"
            {CHARACTER_FRAGMENT}
            query {{
              characters(
                filter: {{
                  gender: "{gender}",
                  name: "{name}",
                  species: "{species}",
                  status: "{status}",
                  type: "{ty}"
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
                  ...CharacterFrag
                }}
              }}
            }}
          "#
        ))
        .map_err(wtx::Error::from)?;
      self.tp.ext_req_params_mut().method = HttpMethod::Post;
      Ok(CharactersReq { operation_name: None, query: buffer, variables: None })
    }
  }

  #[pkg::req_data]
  pub type CharactersReq<'any> = GraphQlRequest<(), &'any str, ()>;

  #[pkg::res_data]
  pub type CharactersRes = GraphQlResponse<CharactersData, serde::de::IgnoredAny>;

  #[derive(Debug, serde::Deserialize)]
  #[doc = generic_data_doc!()]
  pub struct Characters {
    /// Pagination
    pub info: Pagination,
    /// Characters
    pub results: Vec<Character>,
  }

  #[derive(Debug, serde::Deserialize)]
  #[doc = generic_data_doc!()]
  pub struct CharactersData {
    /// Characters
    pub characters: Characters,
  }
}
