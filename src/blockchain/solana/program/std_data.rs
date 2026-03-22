use crate::blockchain::solana::{
  misc::sub_slice,
  program::{LenBounds, PackData, TestingInstances},
};
use wtx::collection::Vector;

macro_rules! create_and_impl_std_data {
  ($struct_be:ident, $struct_le:ident, $elem:ty, $bytes:literal) => {
    // Big-ending

    create_data_struct! {
      #[derive(Debug, Default, Eq, PartialEq)]
      pub struct $struct_be<> {
        pub elem: $elem
      }
    }

    impl From<$elem> for $struct_be {
      fn from(from: $elem) -> Self {
        $struct_be { elem: from }
      }
    }

    impl From<$struct_be> for $elem {
      fn from(from: $struct_be) -> Self {
        from.elem
      }
    }

    impl PackData for $struct_be {
      const LEN_BOUNDS: LenBounds = LenBounds::from_same($bytes);

      fn pack_data(&self, bytes: &mut Vector<u8>) -> crate::Result<()> {
        bytes.extend_from_copyable_slice(&self.elem.to_be_bytes())?;
        Ok(())
      }

      fn unpack_data(bytes: &[u8]) -> crate::Result<Self> {
        let opt = sub_slice(bytes, 0..$bytes).try_into().ok().map(<$elem>::from_be_bytes);
        Ok(Self { elem: opt.ok_or(crate::Error::SolanaInvalidAccountData)? })
      }
    }

    impl TestingInstances for $struct_be {
      fn min_instance() -> crate::Result<Self> {
        Ok(Self::from($bytes))
      }
    }

    // Little-ending

    create_data_struct! {
      #[derive(Debug, Default, Eq, PartialEq)]
      pub struct $struct_le<> {
        pub elem: $elem
      }
    }

    impl From<$elem> for $struct_le {
      fn from(from: $elem) -> Self {
        $struct_le { elem: from }
      }
    }

    impl From<$struct_le> for $elem {
      fn from(from: $struct_le) -> Self {
        from.elem
      }
    }

    impl PackData for $struct_le {
      const LEN_BOUNDS: LenBounds = LenBounds::from_same($bytes);

      fn pack_data(&self, bytes: &mut Vector<u8>) -> crate::Result<()> {
        bytes.extend_from_copyable_slice(&self.elem.to_le_bytes())?;
        Ok(())
      }

      fn unpack_data(bytes: &[u8]) -> crate::Result<Self> {
        let opt = sub_slice(bytes, 0..$bytes).try_into().ok().map(<$elem>::from_le_bytes);
        Ok(Self { elem: opt.ok_or(crate::Error::SolanaInvalidAccountData)? })
      }
    }

    impl TestingInstances for $struct_le {
      fn min_instance() -> crate::Result<Self> {
        Ok(Self::from($bytes))
      }
    }

    // Primitives (little-ending)

    impl PackData for $elem {
      const LEN_BOUNDS: LenBounds = LenBounds::from_same($bytes);

      fn pack_data(&self, bytes: &mut Vector<u8>) -> crate::Result<()> {
        $struct_le::from(*self).pack_data(bytes)
      }

      fn unpack_data(bytes: &[u8]) -> crate::Result<Self> {
        Ok($struct_le::unpack_data(bytes)?.into())
      }
    }

    impl TestingInstances for $elem {
      fn min_instance() -> crate::Result<Self> {
        Ok($bytes)
      }
    }
  };
}

create_and_impl_std_data!(I8GE, I8LE, i8, 1);
create_and_impl_std_data!(I16GE, I16LE, i16, 2);
create_and_impl_std_data!(I32GE, I32LE, i32, 4);
create_and_impl_std_data!(I64GE, I64LE, i64, 8);
create_and_impl_std_data!(I128GE, I128LE, i128, 16);

create_and_impl_std_data!(U8GE, U8LE, u8, 1);
create_and_impl_std_data!(U16GE, U16LE, u16, 2);
create_and_impl_std_data!(U32GE, U32LE, u32, 4);
create_and_impl_std_data!(U64GE, U64LE, u64, 8);
create_and_impl_std_data!(U128GE, U128LE, u128, 16);

impl PackData for bool {
  const LEN_BOUNDS: LenBounds = LenBounds::from_same(1);

  fn pack_data(&self, bytes: &mut Vector<u8>) -> crate::Result<()> {
    bytes.push((*self).into())?;
    Ok(())
  }

  fn unpack_data(bytes: &[u8]) -> crate::Result<Self> {
    Ok(match bytes.first() {
      Some(&0) => false,
      Some(&1) => true,
      _ => return Err(crate::Error::SolanaInvalidAccountData),
    })
  }
}

impl TestingInstances for bool {
  fn min_instance() -> crate::Result<Self> {
    Ok(true)
  }
}
