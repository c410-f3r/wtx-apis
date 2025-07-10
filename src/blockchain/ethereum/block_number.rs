/// Block Number
#[derive(Debug)]
pub enum BlockNumber {
  /// Earliest block (genesis)
  Earliest,
  /// Latest block
  Latest,
  /// Block by number from canon chain
  Number(u64),
  /// Pending block (not yet part of the blockchain)
  Pending,
}

impl<T> From<T> for BlockNumber
where
  T: Into<u64>,
{
  #[inline]
  fn from(num: T) -> Self {
    BlockNumber::Number(num.into())
  }
}

mod serde {
  use crate::blockchain::ethereum::BlockNumber;
  use serde::{Serialize, Serializer};
  use wtx::de::{HexDisplay, u64_string};

  impl Serialize for BlockNumber {
    #[inline]
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
      S: Serializer,
    {
      match *self {
        BlockNumber::Number(ref x) => {
          let s = u64_string(*x);
          serializer.collect_str(&format_args!("{}", HexDisplay::<true>(s.as_bytes())))
        }
        BlockNumber::Latest => serializer.serialize_str("latest"),
        BlockNumber::Earliest => serializer.serialize_str("earliest"),
        BlockNumber::Pending => serializer.serialize_str("pending"),
      }
    }
  }
}
