use crate::blockchain::ethereum::{Address, Eip712Domain};
use k256::U256;

pub(crate) fn eip712_domain(chain_id: u16) -> Eip712Domain<'static> {
  Eip712Domain::new(
    Some("AsterSignTransaction"),
    Some("1"),
    Some(U256::from_u16(chain_id)),
    Some(Address([0; 20])),
    None,
  )
}
