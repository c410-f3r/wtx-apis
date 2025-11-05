use serde::{Serializer, ser::SerializeStruct};

pub(crate) fn serialize_sig<S>(sig: &Signature, s: S) -> std::result::Result<S::Ok, S::Error>
where
  S: Serializer,
{
  let mut state = s.serialize_struct("Signature", 3)?;
  state.serialize_field("r", &sig.r())?;
  state.serialize_field("s", &sig.s())?;
  state.serialize_field("v", &27u64.wrapping_add(sig.v()))?;
  state.end()
}
