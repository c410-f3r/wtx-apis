#[wtx::pkg(data_format(json), id(crate::series::rick_and_morty::RickAndMortyId), transport(http))]
pub(crate) mod pkg {
  use crate::series::rick_and_morty::{CHARACTER_FRAGMENT, Character, RickAndMortyHttpPkgsAux};
  use alloc::string::String;
  use core::fmt::Write;
  use wtx::{
    client_api_framework::network::transport::TransportParams,
    de::protocol::{GraphQlDecoder, GraphQlEncoder},
    http::Method,
  };

  #[pkg::aux]
  impl<DRSR> RickAndMortyHttpPkgsAux<DRSR> {
    #[pkg::aux_data]
    fn character_data<'any>(
      &mut self,
      buffer: &'any mut String,
      id: u32,
    ) -> crate::Result<CharacterReq<'any>> {
      buffer.clear();
      buffer
        .write_fmt(format_args!(
          r#"
            {CHARACTER_FRAGMENT}
            query {{
              character(id: "{id}") {{
                ...CharacterFrag
              }}
            }}
          "#
        ))
        .map_err(wtx::Error::from)?;
      self.tp.ext_req_params_mut().method = Method::Post;
      Ok(CharacterReq { operation_name: None, query: buffer, variables: None })
    }
  }

  #[pkg::req_data]
  pub type CharacterReq<'any> = GraphQlEncoder<(), &'any str, ()>;

  #[pkg::res_data]
  pub type CharacterRes<'any> = GraphQlDecoder<CharacterData<&'any str>, serde::de::IgnoredAny>;

  #[derive(Debug, serde::Deserialize)]
  #[doc = generic_data_doc!()]
  pub struct CharacterData<T> {
    /// Character
    pub character: Character<T>,
  }
}
