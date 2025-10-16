mod albums;
mod comments;
mod photos;
mod posts;
mod todos;
mod users;

pub use albums::pkg::*;
pub use comments::pkg::*;
pub use photos::pkg::*;
pub use posts::pkg::*;
pub use todos::pkg::*;
pub use users::pkg::*;

use alloc::boxed::Box;
use wtx::{
  client_api_framework::network::{HttpParams, transport::TransportParams},
  collection::Vector,
  http::Method,
};

/// Generic response used by all packages.
#[derive(Debug, serde::Deserialize)]
#[serde(untagged)]
pub enum GenericRes<T> {
  /// One album.
  Album(Box<Album<T>>),
  /// Multiple albums.
  Albums(Vector<Album<T>>),
  /// One comment.
  Comment(Box<Comment<T>>),
  /// Multiple comments.
  Comments(Vector<Comment<T>>),
  /// One photo.
  Photo(Box<Photo<T>>),
  /// Multiple photos.
  Photos(Vector<Photo<T>>),
  /// One post.
  Post(Box<Post<T>>),
  /// Multiple posts.
  Posts(Vector<Post<T>>),
  /// One todo.
  Todo(Box<Todo<T>>),
  /// Multiple todos.
  Todos(Vector<Todo<T>>),
  /// One user.
  User(Box<User<T>>),
  /// Multiple users.
  Users(Vector<User<T>>),
}

/// Generic parameters used by all packages.
#[derive(Debug)]
pub struct GenericParams<'any> {
  id_opt: Option<u32>,
  method: Method,
  nested_opt: Option<&'any str>,
  query: &'any [(&'any str, &'any str)],
}

impl<'any> GenericParams<'any> {
  /// Constructor shortcut
  #[inline]
  pub const fn new(
    id_opt: Option<u32>,
    method: Method,
    nested_opt: Option<&'any str>,
    query: &'any [(&'any str, &'any str)],
  ) -> Self {
    Self { id_opt, method, nested_opt, query }
  }

  pub(crate) fn manage(
    &mut self,
    endpoint: &str,
    trans_params: &mut HttpParams,
  ) -> crate::Result<()> {
    trans_params.ext_req_params_mut().method = self.method;
    match (self.id_opt, self.nested_opt) {
      (None, None) | (None, Some(_)) => {
        trans_params.ext_req_params_mut().uri.push_path(format_args!("/{endpoint}"))?
      }
      (Some(id), None) => {
        trans_params.ext_req_params_mut().uri.push_path(format_args!("/{endpoint}/{id}"))?
      }
      (Some(id), Some(nested)) => trans_params
        .ext_req_params_mut()
        .uri
        .push_path(format_args!("/{endpoint}/{id}/{nested}"))?,
    }
    if let [first, rest @ ..] = self.query {
      let mut qw = trans_params.ext_req_params_mut().uri.query_writer(first.0, first.1)?;
      for (key, value) in rest {
        qw = qw.write(key, value)?;
      }
    }
    Ok(())
  }
}
