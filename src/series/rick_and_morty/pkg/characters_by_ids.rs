#[wtx::pkg(data_format(json), id(crate::series::rick_and_morty::RickAndMortyId), transport(http))]
pub(crate) mod pkg {
  use crate::{
    misc::SliceByCommas,
    series::rick_and_morty::{CHARACTER_FRAGMENT, Character, RickAndMortyHttpPkgsAux},
  };
  use alloc::string::String;
  use core::fmt::Write;
  use wtx::{
    client_api_framework::network::transport::TransportParams,
    collection::Vector,
    de::protocol::{GraphQlDecoder, GraphQlEncoder},
    http::Method,
  };

  #[pkg::aux]
  impl<DRSR> RickAndMortyHttpPkgsAux<DRSR> {
    #[pkg::aux_data]
    fn characters_by_ids_data<'any>(
      &mut self,
      buffer: &'any mut String,
      ids: &[u32],
    ) -> crate::Result<CharactersByIdsReq<'any>> {
      buffer.clear();
      buffer
        .write_fmt(format_args!(
          r#"
            {CHARACTER_FRAGMENT}
            query {{
              charactersByIds(ids: ["{}"]) {{
                ...CharacterFrag
              }}
            }}
          "#,
          SliceByCommas(ids)
        ))
        .map_err(wtx::Error::from)?;
      self.tp.ext_req_params_mut().method = Method::Post;
      Ok(CharactersByIdsReq { operation_name: None, query: buffer, variables: None })
    }
  }

  #[pkg::req_data]
  pub type CharactersByIdsReq<'any> = GraphQlEncoder<(), &'any str, ()>;

  #[pkg::res_data]
  pub type CharactersByIdsRes<'any> =
    GraphQlDecoder<CharactersByIdsData<&'any str>, serde::de::IgnoredAny>;

  #[derive(Debug, serde::Deserialize)]
  #[doc = generic_data_doc!()]
  #[serde(rename_all = "camelCase")]
  pub struct CharactersByIdsData<T> {
    /// Characters by ids
    pub characters_by_ids: Vector<Character<T>>,
  }
}
