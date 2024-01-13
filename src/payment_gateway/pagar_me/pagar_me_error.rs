use alloc::{boxed::Box, string::String};
use wtx::http::Method;

#[derive(Debug, serde::Deserialize)]
pub struct PagarMeErrors {
  pub errors: Box<[PagarMeError]>,
  pub method: Method,
  pub url: Box<str>,
}

#[derive(Debug, serde::Deserialize)]
pub struct PagarMeError {
  pub message: String,
  pub parameter_name: Option<String>,
  #[serde(rename = "type")]
  pub ty: String,
}
