use crate::exchange::extended::Balance;
use alloc::boxed::Box;
use core::marker::PhantomData;
use serde::{
  Deserialize, Deserializer,
  de::{MapAccess, Visitor},
};

#[derive(Clone, Copy, Debug, PartialEq, Eq, serde::Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum HttpResponseStatus {
  Ok,
  Error,
}

#[derive(Debug, serde::Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum StreamDataType {
  Balance,
  Delta,
  Deposit,
  Mp,
  Order,
  Position,
  Snapshot,
  Trade,
  Transfer,
  Withdrawal,
  Unknown,
}

#[derive(Debug)]
pub struct HttpResponse<T> {
  pub status: HttpResponseStatus,
  pub result: crate::Result<T>,
  pub pagination: Option<HttpResponsePagination>,
}

impl<'de, T> Deserialize<'de> for HttpResponse<T>
where
  T: Deserialize<'de>,
{
  #[inline]
  fn deserialize<DE>(deserializer: DE) -> Result<HttpResponse<T>, DE::Error>
  where
    DE: Deserializer<'de>,
  {
    #[derive(Debug, serde::Deserialize)]
    #[serde(field_identifier, rename_all = "lowercase")]
    enum Field {
      Status,
      Data,
      Error,
      Pagination,
    }

    struct CustomVisitor<'de, T>(PhantomData<T>, PhantomData<&'de ()>)
    where
      T: Deserialize<'de>;

    impl<'de, T> Visitor<'de> for CustomVisitor<'de, T>
    where
      T: Deserialize<'de>,
    {
      type Value = HttpResponse<T>;

      fn expecting(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        formatter.write_str("struct HttpResponse")
      }

      #[inline]
      fn visit_map<V>(self, mut map: V) -> Result<HttpResponse<T>, V::Error>
      where
        V: MapAccess<'de>,
      {
        let mut data = None;
        let mut status = None;
        let mut error: Option<HttpResponseError<Box<str>>> = None;
        let mut pagination = None;

        while let Some(key) = map.next_key()? {
          match key {
            Field::Status => {
              if status.is_some() {
                return Err(serde::de::Error::duplicate_field("status"));
              }
              status = Some(map.next_value()?);
            }
            Field::Data => {
              if data.is_some() {
                return Err(serde::de::Error::duplicate_field("data"));
              }
              data = Some(map.next_value()?);
            }
            Field::Error => {
              if error.is_some() {
                return Err(serde::de::Error::duplicate_field("error"));
              }
              error = Some(map.next_value()?);
            }
            Field::Pagination => {
              if pagination.is_some() {
                return Err(serde::de::Error::duplicate_field("pagination"));
              }
              pagination = Some(map.next_value()?);
            }
          }
        }

        Ok(HttpResponse {
          status: if let Some(elem) = status {
            elem
          } else {
            return Err(serde::de::Error::missing_field("data"));
          },
          result: match (data, error) {
            (None, Some(elem)) => Err(crate::Error::ExtendedHttpResError(elem.into())),
            (Some(elem), None) => Ok(elem),
            _ => {
              return Err(serde::de::Error::custom(
                "there can only be one data or one error field",
              ));
            }
          },
          pagination,
        })
      }
    }

    deserializer.deserialize_struct(
      "HttpResponse",
      &["status", "data", "error", "pagination"],
      CustomVisitor(PhantomData, PhantomData),
    )
  }
}

#[derive(Debug, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct HttpResponseError<S> {
  pub code: i32,
  pub message: S,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub debug_info: Option<S>,
}

#[derive(Debug, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct HttpResponsePagination {
  #[serde(skip_serializing_if = "Option::is_none")]
  pub cursor: Option<i32>,
  pub count: i32,
}

#[derive(Debug)]
pub struct StreamResponse<T> {
  pub ty: Option<StreamDataType>,
  pub result: crate::Result<T>,
  pub ts: u64,
  pub seq: u64,
}

impl<'de, T> Deserialize<'de> for StreamResponse<T>
where
  T: Deserialize<'de>,
{
  #[inline]
  fn deserialize<DE>(deserializer: DE) -> Result<StreamResponse<T>, DE::Error>
  where
    DE: Deserializer<'de>,
  {
    #[derive(Debug, serde::Deserialize)]
    #[serde(field_identifier, rename_all = "camelCase")]
    enum Field {
      Type,
      Data,
      Error,
      Ts,
      Seq,
      SourceEventId,
    }

    struct CustomVisitor<'de, T>(PhantomData<T>, PhantomData<&'de ()>)
    where
      T: Deserialize<'de>;

    impl<'de, T> Visitor<'de> for CustomVisitor<'de, T>
    where
      T: Deserialize<'de>,
    {
      type Value = StreamResponse<T>;

      fn expecting(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        formatter.write_str("struct StreamResponse")
      }

      #[inline]
      fn visit_map<V>(self, mut map: V) -> Result<StreamResponse<T>, V::Error>
      where
        V: MapAccess<'de>,
      {
        let mut ty = None;
        let mut data = None;
        let mut error: Option<Box<str>> = None;
        let mut ts = None;
        let mut seq = None;

        while let Some(key) = map.next_key()? {
          match key {
            Field::Type => {
              if ty.is_some() {
                return Err(serde::de::Error::duplicate_field("type"));
              }
              ty = Some(map.next_value()?);
            }
            Field::Data => {
              if data.is_some() {
                return Err(serde::de::Error::duplicate_field("data"));
              }
              data = Some(map.next_value()?);
            }
            Field::Error => {
              if error.is_some() {
                return Err(serde::de::Error::duplicate_field("error"));
              }
              error = Some(map.next_value()?);
            }
            Field::Ts => {
              if ts.is_some() {
                return Err(serde::de::Error::duplicate_field("ts"));
              }
              ts = Some(map.next_value()?);
            }
            Field::Seq => {
              if seq.is_some() {
                return Err(serde::de::Error::duplicate_field("seq"));
              }
              seq = Some(map.next_value()?);
            }
            Field::SourceEventId => {
              let _value: u64 = map.next_value()?;
            }
          }
        }

        Ok(StreamResponse {
          ty,
          result: match (data, error) {
            (None, Some(elem)) => Err(crate::Error::ExtendedStreamResError(elem.into())),
            (Some(elem), None) => Ok(elem),
            _ => {
              return Err(serde::de::Error::custom(
                "there can only be one data or one error field",
              ));
            }
          },
          ts: if let Some(elem) = ts {
            elem
          } else {
            return Err(serde::de::Error::missing_field("ts"));
          },
          seq: if let Some(elem) = seq {
            elem
          } else {
            return Err(serde::de::Error::missing_field("seq"));
          },
        })
      }
    }

    deserializer.deserialize_struct(
      "StreamResponse",
      &["type", "data", "error", "ts", "seq", "sourceEventId"],
      CustomVisitor(PhantomData, PhantomData),
    )
  }
}

#[derive(Debug, serde::Deserialize)]
pub struct StreamResponseBalance {
  pub balance: Balance,
}

#[derive(Debug, serde::Deserialize)]
pub struct StreamResponseOrders<O> {
  pub orders: O,
}
