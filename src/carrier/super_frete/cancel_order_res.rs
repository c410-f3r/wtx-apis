use core::marker::PhantomData;
use serde::{
  Deserialize, Deserializer,
  de::{Error, MapAccess, Visitor},
};

/// Cancel order response
#[derive(Debug)]
pub struct CancelOrderResGeneric<S> {
  /// Id
  pub id: S,
  /// Parameters
  pub params: CancelOrderResParams,
}

impl<'de, S> Deserialize<'de> for CancelOrderResGeneric<S>
where
  S: Deserialize<'de>,
{
  #[inline]
  fn deserialize<DE>(deserializer: DE) -> Result<CancelOrderResGeneric<S>, DE::Error>
  where
    DE: Deserializer<'de>,
  {
    struct CustomVisitor<'de, S>(PhantomData<S>, PhantomData<&'de ()>)
    where
      S: Deserialize<'de>;

    impl<'de, S> Visitor<'de> for CustomVisitor<'de, S>
    where
      S: Deserialize<'de>,
    {
      type Value = CancelOrderResGeneric<S>;

      fn expecting(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        formatter.write_str("struct CancelOrderResGeneric")
      }

      #[inline]
      fn visit_map<V>(self, mut map: V) -> Result<CancelOrderResGeneric<S>, V::Error>
      where
        V: MapAccess<'de>,
      {
        if let Some(id) = map.next_key()? {
          let params = map.next_value()?;
          return Ok(CancelOrderResGeneric { id, params });
        }
        Err(V::Error::custom("Cancel response must have one parameter"))
      }
    }

    deserializer.deserialize_struct(
      "CancelOrderResGeneric",
      &["id"],
      CustomVisitor(PhantomData, PhantomData),
    )
  }
}

/// Parameter
#[derive(Debug, serde::Deserialize)]
pub struct CancelOrderResParams {
  /// Is canceled
  pub canceled: bool,
}
