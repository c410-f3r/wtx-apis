use crate::{
  blockchain::solana::{
    misc::sub_slice,
    program::{LenBounds, PackData, TestingInstances},
  },
  misc::HashArray32Unit,
};
use wtx::collection::Vector;

create_data_struct! {
  #[derive(Debug, Eq, PartialEq)]
  pub struct NewAddresses<H> {
    pub new_addresses: Vector<H>
  }
}

impl PackData for NewAddresses<HashArray32Unit> {
  const LEN_BOUNDS: LenBounds = LenBounds::new(8, None);

  fn len(&self) -> usize {
    self.new_addresses.len().checked_mul(32).and_then(|el| el.checked_add(8)).unwrap_or(usize::MAX)
  }

  fn pack_data(&self, buffer: &mut Vector<u8>) -> crate::Result<()> {
    let Self { new_addresses } = self;
    u64::try_from(new_addresses.len()).map_err(wtx::Error::from)?.pack_data(buffer)?;
    for hash in new_addresses.iter() {
      buffer.extend_from_copyable_slice(hash.bytes())?;
    }
    Ok(())
  }

  fn unpack_data(bytes: &[u8]) -> crate::Result<Self> {
    let mut end = 8;
    let len =
      usize::try_from(u64::unpack_data(sub_slice(bytes, 0..end))?).map_err(wtx::Error::from)?;
    let mut new_addresses = Vector::with_capacity(len)?;
    for _ in 0..len {
      let local_start = end;
      let local_end = local_start.wrapping_add(32);
      new_addresses.push(HashArray32Unit::from_bytes(
        sub_slice(bytes, local_start..local_end).try_into().map_err(wtx::Error::from)?,
      ))?;
      end = local_end;
    }
    Ok(NewAddresses { new_addresses })
  }
}

impl TestingInstances for NewAddresses<HashArray32Unit> {
  fn min_instance() -> crate::Result<Self> {
    Ok(Self { new_addresses: Vector::new() })
  }

  fn variable_instance() -> crate::Result<Self> {
    Ok(Self { new_addresses: Vector::from_iterator([HashArray32Unit::from_bytes([1; 32])])? })
  }
}
