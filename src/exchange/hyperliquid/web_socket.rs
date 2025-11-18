use core::marker::PhantomData;

use alloc::string::String;
use serde::{
  Deserialize, Deserializer, Serializer,
  de::{MapAccess, Visitor},
};
use wtx::http::Method;

/// WebSocket Request Channel
#[derive(Debug, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub enum WebSocketChannel {
  /// Error
  Error,
  /// Post
  Post,
}

/// WebSocket Request Type
#[derive(Debug, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub enum WebSocketTy {
  /// Action
  Action,
  /// Info
  Info,
}

/// WebSocket Request
#[derive(Debug, serde::Serialize)]
pub struct WebSocketReq<P> {
  /// Increasing ID
  pub id: u64,
  /// See [`Method`].
  #[serde(serialize_with = "serialize_method")]
  pub method: Method,
  /// See [`WebSocketReqParams`].
  pub request: WebSocketReqParams<P>,
}

/// WebSocket Request Parameters
#[derive(Debug, serde::Serialize)]
pub struct WebSocketReqParams<P> {
  /// See [`WebSocketReqTy`].
  #[serde(rename = "type")]
  pub ty: WebSocketTy,
  /// Opaque payload
  pub payload: P,
}

/// WebSocket Response
#[derive(Debug)]
pub struct WebSocketRes<P> {
  /// See [`WebSocketResData`].
  pub result: Result<WebSocketResData<P>, String>,
}

impl<'de, P> Deserialize<'de> for WebSocketRes<P>
where
  P: Deserialize<'de>,
{
  #[inline]
  fn deserialize<DE>(deserializer: DE) -> Result<WebSocketRes<P>, DE::Error>
  where
    DE: Deserializer<'de>,
  {
    #[derive(Debug, serde::Deserialize)]
    #[serde(field_identifier, rename_all = "lowercase")]
    enum Field {
      Channel,
      Data,
    }

    struct LocalVisitor<P>(PhantomData<P>);

    impl<'de, P> Visitor<'de> for LocalVisitor<P>
    where
      P: Deserialize<'de>,
    {
      type Value = WebSocketRes<P>;

      fn expecting(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        formatter.write_str("struct WebSocketRes")
      }

      #[inline]
      fn visit_map<V>(self, mut map: V) -> Result<WebSocketRes<P>, V::Error>
      where
        V: MapAccess<'de>,
      {
        let mut ok = None;
        let mut err = None;

        'parse: {
          let Some(Field::Channel) = map.next_key::<Field>()? else {
            break 'parse;
          };
          let channel_str = map.next_value::<&str>()?;
          let is_ok = channel_str != "error";
          let Some(Field::Data) = map.next_key::<Field>()? else {
            break 'parse;
          };
          if is_ok { ok = map.next_value()? } else { err = map.next_value()? }
        }

        if map.next_key::<Field>()?.is_some() {
          return Err(serde::de::Error::invalid_length(3, &"2"));
        }

        Ok(WebSocketRes {
          result: if let Some(elem) = err {
            Err(elem)
          } else {
            Ok(ok.ok_or_else(|| serde::de::Error::missing_field("data"))?)
          },
        })
      }
    }

    deserializer.deserialize_struct("WebSocketRes", &["channel", "data"], LocalVisitor(PhantomData))
  }
}

/// WebSocket Response Data
#[derive(Debug, serde::Deserialize)]
pub struct WebSocketResData<P> {
  /// Increasing ID
  pub id: u64,
  /// See [`WebSocketResParams`].
  pub response: WebSocketResParams<P>,
}

/// WebSocket Response Parameters
#[derive(Debug, serde::Deserialize)]
pub struct WebSocketResParams<P> {
  /// See [`WebSocketTy`].
  #[serde(rename = "type")]
  pub ty: WebSocketTy,
  /// Opaque payload
  pub payload: P,
}

fn serialize_method<S>(method: &Method, s: S) -> Result<S::Ok, S::Error>
where
  S: Serializer,
{
  s.collect_str(method.strings().custom[1])
}
