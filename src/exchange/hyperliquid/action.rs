use crate::{
  blockchain::ethereum::misc::keccak256,
  exchange::hyperliquid::{
    BulkCancel, BulkCancelCloid, BulkModify, BulkOrder, SpotSend, SpotUser, usd_send::UsdSend,
  },
};
use wtx::collection::Vector;

#[derive(Debug, serde::Serialize)]
#[serde(tag = "type")]
#[serde(rename_all = "camelCase")]
pub(crate) enum Action<'any> {
  BulkModify(BulkModify<'any>),
  Cancel(BulkCancel<'any>),
  CancelByCloid(BulkCancelCloid<'any>),
  Order(BulkOrder<'any>),
  SpotSend(SpotSend<'any>),
  SpotUser(SpotUser),
  UsdSend(UsdSend<'any>),
}

impl<'any> Action<'any> {
  pub(crate) fn hash(
    &self,
    buffer: &mut Vector<u8>,
    timestamp: u64,
    vault_address: Option<&[u8]>,
  ) -> crate::Result<[u8; 32]> {
    buffer.clear();
    rmp_serde::encode::write_named(buffer, self)?;
    if let Some(elem) = vault_address {
      buffer.extend_from_copyable_slices([timestamp.to_be_bytes().as_ref(), &[1u8][..], elem])?;
    } else {
      buffer.extend_from_copyable_slices([timestamp.to_be_bytes().as_ref(), &[0u8][..]])?;
    }
    Ok(keccak256([&*buffer]))
  }
}

macro_rules! impl_froms {
  ($(($variant:ident, $from:ty)),* $(,)?) => {
    $(
      impl<'any> From<$from> for Action<'any> {
        fn from(value: $from) -> Self {
          Self::$variant(value)
        }
      }
    )*
  };
}

impl_froms!(
  (BulkModify, BulkModify<'any>),
  (Cancel, BulkCancel<'any>),
  (CancelByCloid, BulkCancelCloid<'any>),
  (Order, BulkOrder<'any>),
  (SpotSend, SpotSend<'any>),
  (SpotUser, SpotUser),
  (UsdSend, UsdSend<'any>),
);
