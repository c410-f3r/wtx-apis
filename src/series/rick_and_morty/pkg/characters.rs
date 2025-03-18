#[wtx_macros::pkg(
  data_format(json),
  id(crate::series::rick_and_morty::RickAndMortyId),
  transport(http)
)]
pub(crate) mod pkg {
  use crate::series::rick_and_morty::{
    CHARACTER_FRAGMENT, Character, Pagination, RickAndMortyHttpPkgsAux,
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
      self.tp.ext_req_params_mut().method = Method::Post;
      Ok(CharactersReq { operation_name: None, query: buffer, variables: None })
    }
  }

  #[pkg::req_data]
  pub type CharactersReq<'any> = GraphQlRequest<(), &'any str, ()>;

  #[pkg::res_data]
  pub type CharactersRes<'any> = GraphQlResponse<CharactersData<&'any str>, serde::de::IgnoredAny>;

  #[derive(Debug, serde::Deserialize)]
  #[doc = generic_data_doc!()]
  pub struct Characters<T> {
    /// Pagination
    pub info: Pagination<T>,
    /// Characters
    pub results: Vector<Character<T>>,
  }

  #[derive(Debug, serde::Deserialize)]
  #[doc = generic_data_doc!()]
  pub struct CharactersData<T> {
    /// Characters
    pub characters: Characters<T>,
  }
}
