#[wtx_macros::pkg(
  data_format(json),
  id(crate::series::rick_and_morty::RickAndMortyId),
  transport(http)
)]
pub(crate) mod pkg {
  use crate::{
    misc::SliceByCommas,
    series::rick_and_morty::{CHARACTER_FRAGMENT, Episode, RickAndMortyHttpPkgsAux},
  };
  use alloc::string::String;
  use core::fmt::Write;
  use wtx::{
    client_api_framework::network::transport::TransportParams,
    collection::Vector,
    data_transformation::format::{GraphQlRequest, GraphQlResponse},
    http::Method,
  };

  #[pkg::aux]
  impl<DRSR> RickAndMortyHttpPkgsAux<DRSR> {
    #[pkg::aux_data]
    fn episodes_by_ids_data<'any>(
      &mut self,
      buffer: &'any mut String,
      ids: &[u32],
    ) -> crate::Result<EpisodesByIdsReq<'any>> {
      buffer.clear();
      buffer
        .write_fmt(format_args!(
          r#"
            {CHARACTER_FRAGMENT}
            query {{
              episodesByIds(ids: ["{}"]) {{
                  air_date
                  characters {{
                    ...CharacterFrag
                  }}
                  created
                  episode
                  id
                  name
              }}
            }}
          "#,
          SliceByCommas(ids)
        ))
        .map_err(wtx::Error::from)?;
      self.tp.ext_req_params_mut().method = Method::Post;
      Ok(EpisodesByIdsReq { operation_name: None, query: buffer, variables: None })
    }
  }

  #[pkg::req_data]
  pub type EpisodesByIdsReq<'any> = GraphQlRequest<(), &'any str, ()>;

  #[pkg::res_data]
  pub type EpisodesByIdsRes<'any> =
    GraphQlResponse<EpisodesByIdsData<&'any str>, serde::de::IgnoredAny>;

  #[derive(Debug, serde::Deserialize)]
  #[doc = generic_data_doc!()]
  #[serde(rename_all = "camelCase")]
  pub struct EpisodesByIdsData<T> {
    /// Episodes by ids
    pub episodes_by_ids: Vector<Episode<T>>,
  }
}
