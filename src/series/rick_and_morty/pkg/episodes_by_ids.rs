#[wtx_macros::pkg(
  api(crate::series::rick_and_morty::RickAndMorty),
  data_format(json),
  error(crate::Error),
  transport(http)
)]
pub(crate) mod pkg {
  use crate::{
    misc::SliceByCommas,
    series::rick_and_morty::{Episode, RickAndMortyHttpPkgsAux, CHARACTER_FRAGMENT},
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
      self.tp.ext_req_params_mut().method = HttpMethod::Post;
      Ok(EpisodesByIdsReq { operation_name: None, query: buffer, variables: None })
    }
  }

  #[pkg::req_data]
  pub type EpisodesByIdsReq<'any> = GraphQlRequest<(), &'any str, ()>;

  #[pkg::res_data]
  pub type EpisodesByIdsRes = GraphQlResponse<EpisodesByIdsData, serde::de::IgnoredAny>;

  #[derive(Debug, serde::Deserialize)]
  #[doc = generic_data_doc!()]
  #[serde(rename_all = "camelCase")]
  pub struct EpisodesByIdsData {
    /// Episodes by ids
    pub episodes_by_ids: Vec<Episode>,
  }
}
