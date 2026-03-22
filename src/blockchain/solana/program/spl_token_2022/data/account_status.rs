use crate::blockchain::solana::program::{LenBounds, PackData};
use wtx::collection::Vector;

create_data_enum! {
  #[derive(Clone, Copy, Debug, Eq, PartialEq)]
  pub enum AccountStatus {
    Uninitialized,
    Initialized,
    Frozen,
  }
}

impl PackData for AccountStatus {
  const LEN_BOUNDS: LenBounds = LenBounds::from_same(1);

  fn pack_data(&self, buffer: &mut Vector<u8>) -> crate::Result<()> {
    match *self {
      Self::Uninitialized => 0u8,
      Self::Initialized => 1,
      Self::Frozen => 2,
    }
    .pack_data(buffer)?;
    Ok(())
  }

  fn unpack_data(bytes: &[u8]) -> crate::Result<Self> {
    Ok(match bytes.first() {
      Some(&0) => Self::Uninitialized,
      Some(&1) => Self::Initialized,
      Some(&2) => Self::Frozen,
      _ => return Err(crate::Error::SolanaInvalidAccountData),
    })
  }
}
