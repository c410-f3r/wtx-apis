use crate::blockchain::solana::{SolanaAddressHash, misc::sub_slice};
use wtx::collection::Vector;

pub(crate) fn pack_coption_address(
  src: Option<&SolanaAddressHash>,
  dst: &mut Vector<u8>,
) -> crate::Result<()> {
  match src {
    None => {
      dst.extend_from_copyable_slice(&[0; 4])?;
    }
    Some(elem) => {
      let _ = dst.extend_from_copyable_slices([&[1, 0, 0, 0][..], elem])?;
    }
  }
  Ok(())
}

pub(crate) fn pack_coption_u64(src: Option<&u64>, buffer: &mut Vector<u8>) -> crate::Result<()> {
  match src {
    None => {
      buffer.extend_from_copyable_slice(&[0; 4])?;
    }
    Some(amount) => {
      let _ = buffer.extend_from_copyable_slices([&[1, 0, 0, 0][..], &amount.to_le_bytes()])?;
    }
  }
  Ok(())
}

pub(crate) fn pack_option_address(
  src: Option<&SolanaAddressHash>,
  dst: &mut Vector<u8>,
) -> crate::Result<()> {
  match src {
    None => {
      dst.push(0)?;
    }
    Some(elem) => {
      dst.push(1)?;
      dst.extend_from_copyable_slice(elem)?;
    }
  }
  Ok(())
}

pub(crate) fn unpack_coption_address(bytes: &[u8]) -> crate::Result<Option<SolanaAddressHash>> {
  let (tag, body) = (sub_slice(bytes, 0..4), sub_slice(bytes, 4..36));
  match *tag {
    [0, 0, 0, 0] => Ok(None),
    [1, 0, 0, 0] => Ok(Some(body.try_into().map_err(wtx::Error::from)?)),
    _ => Err(crate::Error::SolanaInvalidAccountData),
  }
}

pub(crate) fn unpack_coption_u64(bytes: &[u8]) -> crate::Result<Option<u64>> {
  let (tag, body) = (sub_slice(bytes, 0..4), sub_slice(bytes, 4..12));
  match *tag {
    [0, 0, 0, 0] => Ok(None),
    [1, 0, 0, 0] => Ok(Some(u64::from_le_bytes(body.try_into().map_err(wtx::Error::from)?))),
    _ => Err(crate::Error::SolanaInvalidAccountData),
  }
}

pub(crate) fn unpack_option_address(src: &[u8]) -> crate::Result<Option<SolanaAddressHash>> {
  Ok(match src.first() {
    Some(&0) => None,
    Some(&1) => Some(sub_slice(src, 1..33).try_into().map_err(wtx::Error::from)?),
    _ => return Err(crate::Error::SolanaInvalidAccountData),
  })
}
