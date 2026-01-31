//! Decentralized exchange

//#[cfg(all(test, feature = "_integration-tests"))]
//mod integration_tests;
mod order;
mod pkg;
mod sign_params;
mod v1_order_get;
mod v1_order_post;
mod web_socket;

use alloc::string::String;
use core::fmt::Arguments;
use hmac::{Hmac, KeyInit, Mac};
pub use order::*;
pub use pkg::*;
pub use sign_params::SignParams;
pub use v1_order_get::*;
pub use v1_order_post::*;
pub use web_socket::*;
use wtx::{
  client_api_framework::{
    Api,
    misc::RequestCounter,
    network::{HttpParams, HttpReqParams, transport::TransportParams},
  },
  collection::{ArrayVectorU8, Vector},
  de::{HexEncMode, encode_hex},
  http::{Header, Method},
  misc::Secret,
};

/// Production Spot HTTP Uri
pub const PROD_SPOT_HTTP_URI: &str = "https://sapi.asterdex.com";
/// Production Spot WebSocket Uri
pub const PROD_SPOT_WS_URI: &str = "wss://sstream.asterdex.com";

/// Testnet Spot HTTP Uri
pub const TESTNET_SPOT_HTTP_URI: &str = "https://sapi.asterdex-testnet.com";
/// Testnet Spot WebSocket Uri
pub const TESTNET_SPOT_WS_URI: &str = "wss://sstream.asterdex-testnet.com";

/// Aster automatically creates a client order with a maximum length of 22.
pub type ClientOrderIdTy = wtx::collection::ArrayStringU8<22>;

/// Manages endpoints
#[derive(Debug)]
#[wtx::api(error(crate::Error), pkgs_aux(PkgsAux), transport(http, ws))]
pub struct Aster {
  api_key: String,
  rt: RequestCounter,
  secret_key: Secret,
}

impl Aster {
  /// New instance
  pub const fn new(api_key: String, rt: RequestCounter, secret_key: Secret) -> Self {
    Self { api_key, rt, secret_key }
  }

  fn auth_req<const IS_POST: bool>(
    &self,
    bytes_buffer: &mut Vector<u8>,
    path: Arguments<'_>,
    send_bytes_buffer: &mut bool,
    trans_params: &mut HttpParams,
  ) -> crate::Result<()> {
    let HttpReqParams { host, mime, rrb, user_agent_default, .. } =
      trans_params.ext_req_params_mut();
    let _ = bytes_buffer.extend_from_copyable_slices([
      b"&signature=",
      sign(&mut [0; _], bytes_buffer, &self.secret_key)?.as_bytes(),
    ])?;
    if IS_POST {
      rrb.uri.push_path(path)?;
    } else {
      // SAFETY: URL encode is ASCII
      let query = unsafe { core::str::from_utf8_unchecked(bytes_buffer) };
      let rslt = rrb.uri.push_path(format_args!("{path}?{query}"));
      bytes_buffer.clear();
      rslt?;
    }
    rrb.headers.push_from_iter_many([Header::from_name_and_value(
      "x-mbx-apikey",
      [self.api_key.as_str()].into_iter(),
    )])?;
    *host = false;
    *mime = Some(wtx::http::Mime::ApplicationXWwwFormUrlEncoded);
    *send_bytes_buffer = true;
    *user_agent_default = false;
    if IS_POST {
      trans_params.ext_req_params_mut().method = Method::Post;
    }
    Ok(())
  }
}

impl Api for Aster {
  type Error = crate::Error;
  type Id = AsterId;

  async fn before_sending(&mut self) -> Result<(), Self::Error> {
    self.rt.update_params().await?;
    Ok(())
  }
}

wtx::create_packages_aux_wrapper!();

fn sign<'buffer>(
  buffer: &'buffer mut [u8; 64],
  msg: &[u8],
  secret_key: &Secret,
) -> crate::Result<&'buffer str> {
  let array = secret_key.peek(&mut ArrayVectorU8::<_, { 64 + 28 }>::new(), |bytes| {
    let Ok(mut mac) = Hmac::<sha2::Sha256>::new_from_slice(bytes) else {
      return Ok([0; _]);
    };
    mac.update(msg);
    crate::Result::Ok(mac.finalize().into_bytes().into())
  })??;
  Ok(encode_hex(&array, Some(HexEncMode::WithoutPrefixLower), buffer)?)
}

#[cfg(test)]
mod tests {
  use crate::exchange::aster::{Aster, sign};
  use wtx::{
    client_api_framework::misc::{RequestCounter, RequestLimit},
    misc::{Secret, SensitiveBytes},
    rng::{ChaCha20, SeedableRng},
  };

  #[test]
  fn sign_has_correct_output() {
    let mut secret_key = *b"2b5eb11e18796d12d88f13dc27dbbd02c2cc51ff7059765ed9821957d82bb4d9";
    let api = Aster::new(
      "dbefbc809e3e83c283a984c3a1459732ea7db1360ca80c5c2c8867408d28cc83".into(),
      RequestCounter::new(RequestLimit::unlimited()),
      Secret::new(
        SensitiveBytes::new_locked(&mut secret_key[..]).unwrap(),
        &mut ChaCha20::from_std_random().unwrap(),
      )
      .unwrap(),
    );
    assert_eq!(
      sign(
        &mut [0; _],
        b"symbol=BTCUSDT&side=BUY&type=LIMIT&quantity=1&price=9000&timeInForce=GTC&recvWindow=5000&timestamp=1591702613943",
        &api.secret_key
      ).unwrap(),
      "3c661234138461fcc7a7d8746c6558c9842d4e10870d2ecbedf7777cad694af9"
    );
  }
}
