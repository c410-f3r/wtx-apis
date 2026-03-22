use crate::blockchain::solana::program::{AccountAddress, PackAccounts, PackData};
use core::fmt::Debug;
use wtx::collection::Vector;

pub(crate) fn test_account<A, T>(instance: T)
where
  A: AccountAddress + From<[u8; 32]>,
  T: Debug + PackAccounts<A> + PartialEq,
{
  let mut accounts = Vector::new();
  instance.push_accounts(&mut accounts).unwrap();
  assert_eq!(instance.len(), accounts.len());
  let elem = T::unpack_accounts(&mut accounts.iter().map(|el| A::from(el.pubkey))).unwrap();
  assert_eq!(elem, instance);
}

pub(crate) fn test_data<T>(
  instance: T,
  instance_always_equals_min_bound: bool,
  [lesser_variant_len, greater_variant_len]: [&mut usize; 2],
) where
  T: Debug + PackData + PartialEq,
{
  let mut bytes = Vector::new();

  instance.pack_data(&mut bytes).unwrap();

  *greater_variant_len = bytes.len().max(*greater_variant_len);
  *lesser_variant_len = bytes.len().min(*lesser_variant_len);

  if instance_always_equals_min_bound {
    assert_eq!(instance.len(), T::LEN_BOUNDS.min());
  } else {
    assert!(
      instance.len() >= T::LEN_BOUNDS.min(),
      "{} should be >= than {}",
      instance.len(),
      T::LEN_BOUNDS.min()
    );
  }

  assert_eq!(instance.len(), bytes.len());
  assert_eq!(instance, T::unpack_data(&bytes).unwrap());

  if let Some(max) = T::LEN_BOUNDS.max() {
    assert!(instance.len() <= max, "{}, {}", instance.len(), max);
  }

  bytes.clear();
}
