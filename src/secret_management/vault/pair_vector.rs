use core::marker::PhantomData;
use serde::{
  Deserialize,
  de::{Deserializer, Error, MapAccess, Visitor},
};
use wtx::collection::Vector;

/// A vector where each element is a key and a value.
#[derive(Debug)]
pub struct PairVector<K, V>(
  /// Vector
  pub Vector<(K, V)>,
);

impl<'de, K, V> Deserialize<'de> for PairVector<K, V>
where
  K: Deserialize<'de>,
  V: Deserialize<'de>,
{
  #[inline]
  fn deserialize<DE>(deserializer: DE) -> Result<PairVector<K, V>, DE::Error>
  where
    DE: Deserializer<'de>,
  {
    struct CustomVisitor<'de, K, V>(PhantomData<(K, V)>, PhantomData<&'de ()>)
    where
      K: Deserialize<'de>,
      V: Deserialize<'de>;

    impl<'de, K, V> Visitor<'de> for CustomVisitor<'de, K, V>
    where
      K: Deserialize<'de>,
      V: Deserialize<'de>,
    {
      type Value = PairVector<K, V>;

      fn expecting(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        formatter.write_str("struct PairVector")
      }

      #[inline]
      fn visit_map<M>(self, mut map: M) -> Result<PairVector<K, V>, M::Error>
      where
        M: MapAccess<'de>,
      {
        let mut rslt = Vector::new();
        while let Some(key) = map.next_key()? {
          rslt.push((key, map.next_value()?)).map_err(|el| M::Error::custom(el))?;
        }
        Ok(PairVector(rslt))
      }
    }

    deserializer.deserialize_struct("PairVector", &[], CustomVisitor(PhantomData, PhantomData))
  }
}
