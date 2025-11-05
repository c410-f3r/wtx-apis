use crate::exchange::hyperliquid::misc::serialize_sig;
use alloc::string::String;
use core::marker::PhantomData;
use serde::{
  Deserialize, Deserializer,
  de::{MapAccess, Visitor},
};
use wtx::collection::{ArrayStringU8, ArrayVectorU8};

#[derive(Debug, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ExchangePayload<A> {
  action: A,
  #[serde(serialize_with = "serialize_sig")]
  signature: ArrayStringU8<42>,
  nonce: u64,
  vault_address: Option<ArrayStringU8<42>>,
}

#[derive(Debug, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RestingOrder {
  pub oid: u64,
}

#[derive(Debug, serde::Deserialize, serde::Serialize)]
pub struct FilledOrder {
  #[serde(rename = "total_sz")]
  pub total_size: String,
  #[serde(rename = "avg_px")]
  pub avg_price: String,
  pub oid: u64,
}

/// Status of a response
#[derive(Debug, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub enum ExchangeDataStatus {
  /// Success
  Success,
  /// Waiting for fill
  WaitingForFill,
  /// Waiting for trigger
  WaitingForTrigger,
  /// Opaque error
  Error(String),
  /// See [`RestingOrder`].
  Resting(RestingOrder),
  /// See [`FilledOrder`].
  Filled(FilledOrder),
}

/// Type of a response
#[derive(Debug, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub enum ExchangeResponseTy {
  /// Order
  Order,
}

#[derive(Debug, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ExchangeDataStatuses<const N: usize> {
  /// See [`ExchangeDataStatus`].
  pub statuses: ArrayVectorU8<ExchangeDataStatus, N>,
}

#[derive(Debug, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ExchangeResponsePayload<const N: usize> {
  /// See [`ExchangeResponseTy`].
  #[serde(rename = "type")]
  pub ty: ExchangeResponseTy,
  /// See [`ExchangeDataStatuses`].
  pub data: Option<ExchangeDataStatuses<N>>,
}

#[derive(Debug)]
pub struct ExchangeResponse<const N: usize> {
  result: Result<ExchangeResponsePayload<N>, String>,
}

impl<'de, const N: usize> Deserialize<'de> for ExchangeResponse<N> {
  #[inline]
  fn deserialize<DE>(deserializer: DE) -> Result<ExchangeResponse<N>, DE::Error>
  where
    DE: Deserializer<'de>,
  {
    #[derive(Debug, serde::Deserialize)]
    #[serde(field_identifier, rename_all = "lowercase")]
    enum Field {
      Status,
      Response,
    }

    struct LocalVisitor<'de, const N: usize>(PhantomData<&'de ()>);

    impl<'de, const N: usize> Visitor<'de> for LocalVisitor<'de, N> {
      type Value = ExchangeResponse<N>;

      fn expecting(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        formatter.write_str("struct ExchangeResponse")
      }

      #[inline]
      fn visit_map<V>(self, mut map: V) -> Result<ExchangeResponse<N>, V::Error>
      where
        V: MapAccess<'de>,
      {
        let mut ok = None;
        let mut err = None;

        'parse: {
          let Some(Field::Status) = map.next_key::<Field>()? else {
            break 'parse;
          };
          let status_str = map.next_value::<&str>()?;
          let is_ok = match status_str {
            "ok" => true,
            "err" => false,
            _ => return Err(serde::de::Error::missing_field("status")),
          };
          let Some(Field::Response) = map.next_key::<Field>()? else {
            break 'parse;
          };
          if is_ok { ok = map.next_value()? } else { err = map.next_value()? }
        }

        if map.next_key::<Field>()?.is_some() {
          return Err(serde::de::Error::invalid_length(3, &"2"));
        }

        Ok(ExchangeResponse {
          result: if let Some(elem) = err {
            Err(elem)
          } else {
            Ok(ok.ok_or_else(|| serde::de::Error::missing_field("response"))?)
          },
        })
      }
    }

    deserializer.deserialize_struct(
      "ExchangeResponse",
      &["status", "response"],
      LocalVisitor(PhantomData),
    )
  }
}
