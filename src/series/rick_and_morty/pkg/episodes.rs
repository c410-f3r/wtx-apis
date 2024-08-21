#[wtx_macros::pkg(
  api(crate::series::rick_and_morty::RickAndMorty),
  data_format(json),
  transport(http)
)]
pub(crate) mod pkg {
  use crate::series::rick_and_morty::{
    Episode, Pagination, RickAndMortyHttpPkgsAux, CHARACTER_FRAGMENT,
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
    fn episodes_data<'any>(
      &mut self,
      buffer: &'any mut String,
      episode: &str,
      name: &str,
      page: u32,
    ) -> crate::Result<EpisodesReq<'any>> {
      buffer.clear();
      buffer
        .write_fmt(format_args!(
          r#"
            {CHARACTER_FRAGMENT}
            query {{
              episodes(
                filter: {{
                  episode: "{episode}",
                  name: "{name}",
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
            }}
          "#
        ))
        .map_err(wtx::Error::from)?;
      self.tp.ext_req_params_mut().method = Method::Post;
      Ok(EpisodesReq { operation_name: None, query: buffer, variables: None })
    }
  }

  #[pkg::req_data]
  pub type EpisodesReq<'any> = GraphQlRequest<(), &'any str, ()>;

  #[pkg::res_data]
  pub type EpisodesRes = GraphQlResponse<EpisodesData, serde::de::IgnoredAny>;

  #[derive(Debug, serde::Deserialize)]
  #[doc = generic_data_doc!()]
  pub struct Episodes {
    /// Pagination
    pub info: Pagination,
    /// Episodes
    pub results: Vec<Episode>,
  }

  #[derive(Debug, serde::Deserialize)]
  #[doc = generic_data_doc!()]
  pub struct EpisodesData {
    /// Episodes
    pub episodes: Episodes,
  }
}
