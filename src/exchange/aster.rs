//! Decentralized exchange

#[cfg(all(test, feature = "_integration-tests"))]
mod integration_tests;
mod message;
mod order;
mod pkg;
mod sign_params;
mod web_socket;

use crate::{
  blockchain::ethereum::{Address, Eip712Domain, misc::sign_payload},
  exchange::aster::message::Message,
  misc::timestamp_str,
};
use alloc::string::String;
use core::fmt::Arguments;
use crypto_bigint::U256;
use hmac::{Hmac, KeyInit, Mac};
use k256::ecdsa::SigningKey;
pub use order::*;
pub use pkg::*;
use serde::Serialize;
pub use sign_params::CexSignParams;
pub use web_socket::*;
use wtx::{
  client_api_framework::{
    Api,
    misc::RequestCounter,
    network::{HttpParams, HttpReqParams, transport::TransportParams},
  },
  collection::{ArrayVectorU8, Vector},
  de::{AsciiSet, FormUrlSerializer, encode_hex, u64_string},
  http::{Header, Method},
  misc::Secret,
};

const EIP712_DOMAIN: Eip712Domain<'_> = Eip712Domain::new(
  Some("AsterSignTransaction"),
  Some("1"),
  Some(U256::from_u16(714)),
  Some(Address([0; 20])),
  None,
);

/// Production Spot HTTP Uri
pub const PROD_SPOT_HTTP_URI: &str = "https://sapi.asterdex.com";
/// Production Spot WebSocket Uri
pub const PROD_SPOT_WS_URI: &str = "wss://sstream.asterdex.com";

/// Testnet Spot HTTP Uri
pub const TESTNET_SPOT_HTTP_URI: &str = "https://www.asterdex-testnet.com";
/// Testnet Spot WebSocket Uri
pub const TESTNET_SPOT_WS_URI: &str = "wss://sstream.asterdex-testnet.com/stream";

/// Aster automatically creates a client order with a maximum length of 22.
pub type ClientOrderIdTy = wtx::collection::ArrayStringU8<22>;

/// Manages endpoints
#[derive(Debug)]
#[wtx::api(error(crate::Error), pkgs_aux(PkgsAux), transport(http, ws))]
pub struct Aster {
  is_dex: bool,
  rt: RequestCounter,
  secret: Secret,
  signer: String,
  user: String,
}

impl Aster {
  /// New instance
  ///
  /// If `is_dex` is `false`, then `signer` is equal to `api key`, `secret` is equal to `api secret`
  /// and `user` won't be used.
  pub const fn new(
    is_dex: bool,
    rt: RequestCounter,
    secret: Secret,
    signer: String,
    user: String,
  ) -> Self {
    Self { is_dex, rt, secret, signer, user }
  }

  // For some reason `listenKey` does not accept POST contents, as such, all parameters are sent in the URL.
  fn auth_req<const IS_POST: bool, T>(
    &self,
    bytes_buffer: &mut Vector<u8>,
    params: Option<T>,
    path: Arguments<'_>,
    send_bytes_buffer: &mut bool,
    timestamp: Option<u64>,
    tp: &mut HttpParams,
  ) -> crate::Result<()>
  where
    T: Serialize,
  {
    bytes_buffer.clear();
    let HttpReqParams { host, method, mime, rrb, user_agent_default, .. } = tp.ext_req_params_mut();
    let init_char = if let Some(elem) = params {
      elem.serialize(FormUrlSerializer::new(AsciiSet::UNRESERVED, bytes_buffer))?;
      b"&"
    } else {
      &[][..]
    };
    if self.is_dex {
      let timestamp_string = if let Some(elem) = timestamp {
        u64_string(elem)
      } else {
        timestamp_str(|el| el.as_micros())?.1
      };
      let _ = bytes_buffer.extend_from_copyable_slices([
        init_char,
        b"nonce=",
        timestamp_string.as_bytes(),
        b"&user=",
        self.user.as_bytes(),
        b"&signer=",
        self.signer.as_bytes(),
      ])?;
      let signature = self.secret.peek(&mut ArrayVectorU8::<_, { 132 + 28 }>::new(), |pk| {
        crate::Result::Ok(sign_payload(
          &mut rrb.body,
          &Message { msg: bytes_buffer },
          &SigningKey::from_slice(pk)?,
        )?)
      });
      rrb.body.clear();
      let _ = bytes_buffer.extend_from_copyable_slices([
        b"&signature=",
        encode_hex(&signature??.all_bytes(), None, &mut [0; 130])?.as_bytes(),
      ])?;
    } else {
      let timestamp_string = if let Some(elem) = timestamp {
        u64_string(elem)
      } else {
        timestamp_str(|el| el.as_millis())?.1
      };
      let _ = bytes_buffer.extend_from_copyable_slices([
        init_char,
        "timestamp=".as_bytes(),
        timestamp_string.as_bytes(),
      ])?;
      let array = self.secret.peek(&mut ArrayVectorU8::<_, { 64 + 28 }>::new(), |bytes| {
        let Ok(mut mac) = Hmac::<sha2::Sha256>::new_from_slice(bytes) else {
          return Ok([0; _]);
        };
        mac.update(&bytes_buffer);
        crate::Result::Ok(mac.finalize().into_bytes().into())
      })??;
      let _ = bytes_buffer.extend_from_copyable_slices([
        b"&signature=",
        encode_hex(&array, None, &mut [0; 64])?.as_bytes(),
      ])?;
    }
    // SAFETY: URL encoding is ASCII
    let query = unsafe { core::str::from_utf8_unchecked(bytes_buffer) };
    let rslt = rrb.uri.push_path(format_args!("{path}?{query}"));
    bytes_buffer.clear();
    rslt?;
    rrb.headers.push_from_iter_many([Header::from_name_and_value(
      "x-mbx-apikey",
      [self.signer.as_str()].into_iter(),
    )])?;
    *host = false;
    if IS_POST {
      *method = Method::Post;
      *mime = Some(wtx::http::Mime::ApplicationXWwwFormUrlEncoded);
      *send_bytes_buffer = true;
    }
    *user_agent_default = false;
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

#[cfg(test)]
mod tests {
  use crate::exchange::aster::{
    Aster, CexSignParams, OrderPostReqParams, OrderSide, OrderType, TimeInForce,
  };
  use alloc::string::String;
  use rust_decimal::Decimal;
  use wtx::{
    client_api_framework::{
      misc::{RequestCounter, RequestLimit},
      network::{HttpParams, transport::TransportParams},
    },
    collection::Vector,
    de::decode_hex,
    misc::{Secret, SensitiveBytes},
    rng::{ChaCha20, SeedableRng},
  };

  #[test]
  fn sign_cex() {
    let req = OrderPostReqParams {
      symbol: "BTCUSDT".try_into().unwrap(),
      side: OrderSide::Buy,
      ty: OrderType::Limit,
      time_in_force: Some(TimeInForce::Gtc),
      quantity: Some(Decimal::from_parts(1, 0, 0, false, 0)),
      quote_order_qty: None,
      price: Some(Decimal::from_parts(9000, 0, 0, false, 0)),
      new_client_order_id: None,
      stop_price: None,
      cex_sign_params: Some(CexSignParams { recv_window: Some(5000) }),
    };
    let mut secret_key = *b"2b5eb11e18796d12d88f13dc27dbbd02c2cc51ff7059765ed9821957d82bb4d9";
    let api = Aster::new(
      false,
      RequestCounter::new(RequestLimit::unlimited()),
      Secret::new(
        SensitiveBytes::new_locked(&mut secret_key[..]).unwrap(),
        &mut ChaCha20::from_std_random().unwrap(),
      )
      .unwrap(),
      "dbefbc809e3e83c283a984c3a1459732ea7db1360ca80c5c2c8867408d28cc83".into(),
      "".into(),
    );
    let mut tp = HttpParams::from_uri(String::from("hello"));
    api
      .auth_req::<false, _>(
        &mut Vector::new(),
        Some(req),
        format_args!("/world"),
        &mut false,
        Some(1591702613943),
        &mut tp,
      )
      .unwrap();
    assert_eq!(
      tp.ext_params().0.rrb.uri.as_str(),
      "hello/world\
      ?symbol=BTCUSDT\
      &type=LIMIT\
      &side=BUY\
      &timeInForce=GTC\
      &quantity=1\
      &price=9000\
      &recvWindow=5000\
      &timestamp=1591702613943\
      &signature=29ef10476e8cf197381924ff43860f77a4997041eb419521cc4f126bd10ae9a4"
    );
  }

  #[test]
  fn sign_dex() {
    let req = OrderPostReqParams {
      symbol: "ASTERUSDT".try_into().unwrap(),
      side: OrderSide::Buy,
      ty: OrderType::Limit,
      time_in_force: Some(TimeInForce::Gtc),
      quantity: Some(Decimal::from_parts(10, 0, 0, false, 0)),
      quote_order_qty: None,
      price: Some(Decimal::from_parts(6, 0, 0, false, 1)),
      new_client_order_id: None,
      stop_price: None,
      cex_sign_params: None,
    };
    let mut secret_key = [0; 32];
    let _ = decode_hex(
      b"0x4fd0a42218f3eae43a6ce26d22544e986139a01e5b34a62db53757ffca81bae1",
      &mut secret_key,
    )
    .unwrap();
    let api = Aster::new(
      true,
      RequestCounter::new(RequestLimit::unlimited()),
      Secret::new(
        SensitiveBytes::new_locked(&mut secret_key[..]).unwrap(),
        &mut ChaCha20::from_std_random().unwrap(),
      )
      .unwrap(),
      "0x21cF8Ae13Bb72632562c6Fff438652Ba1a151bb0".into(),
      "0x63DD5aCC6b1aa0f563956C0e534DD30B6dcF7C4e".into(),
    );
    let mut tp = HttpParams::from_uri(String::from("hello"));
    api
      .auth_req::<false, _>(
        &mut Vector::new(),
        Some(req),
        format_args!("/world"),
        &mut false,
        Some(1770037768091995),
        &mut tp,
      )
      .unwrap();
    assert_eq!(
      tp.ext_params().0.rrb.uri.as_str(),
      "hello/world\
      ?symbol=ASTERUSDT\
      &type=LIMIT\
      &side=BUY\
      &timeInForce=GTC\
      &quantity=10\
      &price=0.6\
      &nonce=1770037768091995\
      &user=0x63DD5aCC6b1aa0f563956C0e534DD30B6dcF7C4e\
      &signer=0x21cF8Ae13Bb72632562c6Fff438652Ba1a151bb0\
      &signature=293e8438deb0a91c9d9a0851eb4f754da7fc356f13e4937215d05293448ebf975107ad23384f5959d70dd3d1ea6c6f74fabadb7b636f01c982753ec899e810fb1b"
    );
  }
}
