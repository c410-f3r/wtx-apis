mod asset;
mod balance;
mod endpoint_config;
mod fees;
mod market;
mod misc;
mod order;
mod order_book;
mod pkg;
mod poseidon_hasher;
mod position;
mod price_event;
mod response;
mod starknet_domain;
mod trade;
mod trading_fee;
//mod rfc6979;
//mod sign;
//mod fe_utils;

use alloc::string::String;
pub use asset::*;
pub use balance::*;
use core::fmt::Arguments;
pub use endpoint_config::EndpointConfig;
pub use fees::*;
pub use market::*;
pub use misc::*;
pub use order::*;
pub use order_book::*;
pub use poseidon_hasher::PoseidonHasher;
pub use position::*;
pub use price_event::*;
pub use response::*;
pub use starknet_domain::StarknetDomain;
pub use trade::*;
pub use trading_fee::TradingFee;
use wtx::{
  client_api_framework::{
    Api,
    misc::RequestCounter,
    network::{HttpParams, HttpReqParams, transport::TransportParams},
  },
  http::{Header, KnownHeaderName},
};

#[derive(Debug)]
#[wtx::api(error(crate::Error), pkgs_aux(PkgsAux), transport(http, ws))]
pub struct Extended {
  api_key: String,
  rt: RequestCounter,
  user_agent: &'static str,
}

impl Extended {
  pub fn new(api_key: String, rt: RequestCounter, user_agent: &'static str) -> Self {
    Self { api_key, rt, user_agent }
  }

  pub fn api_key(&self) -> &str {
    &self.api_key
  }

  pub fn user_agent(&self) -> &'static str {
    self.user_agent
  }

  fn auth_req(&self, path: Arguments<'_>, trans_params: &mut HttpParams) -> crate::Result<()> {
    let HttpReqParams { host, method: _, mime: _, rrb, user_agent_custom: _, user_agent_default } =
      trans_params.ext_req_params_mut();
    rrb.uri.push_path(path)?;
    rrb.headers.push_from_iter_many([
      Header::from_name_and_value(KnownHeaderName::UserAgent.into(), [self.user_agent].into_iter()),
      Header::from_name_and_value(KnownHeaderName::Host.into(), [rrb.uri.host()].into_iter()),
      Header::new(true, false, "x-api-key", [self.api_key.as_str()].into_iter()),
    ])?;
    *host = false;
    *user_agent_default = false;
    Ok(())
  }
}

impl Api for Extended {
  type Error = crate::Error;
  type Id = ExtendedId;

  async fn before_sending(&mut self) -> Result<(), Self::Error> {
    self.rt.update_params().await?;
    Ok(())
  }
}

wtx::create_packages_aux_wrapper!();
