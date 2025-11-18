use crate::{
  blockchain::ethereum::{Address, Eip712, Eip712Domain, SolTy, encoder::Encoder},
  exchange::hyperliquid::{
    Hyperliquid, PkgsAux, action::Action, agent::Agent, payload::ExchangePayload,
    signature::Signature,
  },
};
use core::{
  fmt::LowerHex,
  mem,
  sync::atomic::{AtomicU64, Ordering},
};
use serde::{Serialize, Serializer, ser::SerializeStruct};
use wtx::{
  calendar::Instant,
  client_api_framework::network::{WsParams, transport::TransportParams},
  collection::{ArrayWrapper, Vector},
  misc::Lease,
};

static CURR_NONCE: AtomicU64 = AtomicU64::new(0);

pub(crate) fn abi_encode_from_buffer<'st, T>(
  buffer: &mut Vector<u8>,
  st: &'st T,
) -> crate::Result<()>
where
  T: SolTy<'st>,
{
  let mut enc = Encoder::from_buffer(mem::take(buffer));
  let rslt = st.abi_encode(&mut enc);
  mem::swap(buffer, enc.buffer_mut());
  rslt?;
  Ok(())
}

pub(crate) fn eip_712_domain(chain_id: u64) -> Eip712Domain<'static> {
  Eip712Domain::new(
    Some("HyperliquidSignTransaction"),
    Some("1"),
    Some(chain_id.into()),
    Some(Address::default()),
    None,
  )
}

pub(crate) fn manage_l1_action<'action, A, DRSR, T>(
  action: Action<'action>,
  pa: &mut PkgsAux<A, DRSR, WsParams>,
  wallet: &k256::ecdsa::SigningKey,
  cb: impl FnOnce(u64, ExchangePayload<Action<'action>>) -> T,
) -> crate::Result<()>
where
  A: Lease<Hyperliquid>,
  T: Serialize,
{
  let wtx::client_api_framework::pkg::PkgsAux {
    api,
    built_requests,
    bytes_buffer,
    drsr: _,
    log_body: _,
    send_bytes_buffer,
    tp: _,
  } = &mut **pa;
  let is_mainnet = api.lease().is_mainnet;
  let nonce = next_nonce()?;
  let connection_id = action.hash(bytes_buffer, nonce, None)?;
  let signature = sign_l1_action(bytes_buffer, connection_id, is_mainnet, wallet)?;
  bytes_buffer.clear();
  let exchange_payload = ExchangePayload { action, signature, nonce, vault_address: None };
  serde_json::to_writer(&mut *bytes_buffer, &cb(*built_requests, exchange_payload))?;
  *send_bytes_buffer = true;
  Ok(())
}

pub(crate) fn manage_typed_data<'pl, A, DRSR, T, TP>(
  nonce: u64,
  pa: &mut PkgsAux<A, DRSR, TP>,
  payload: T,
  wallet: &k256::ecdsa::SigningKey,
) -> crate::Result<()>
where
  A: Lease<Hyperliquid>,
  T: Eip712 + Into<Action<'pl>> + 'pl,
  TP: TransportParams,
{
  let signature = sign_typed_data(&mut pa.bytes_buffer, &payload, wallet)?;
  let action = payload.into();
  let exchange_payload = ExchangePayload { action, signature, nonce, vault_address: None };
  serde_json::to_writer(&mut pa.bytes_buffer, &exchange_payload)?;
  pa.send_bytes_buffer = true;
  Ok(())
}

pub(crate) fn next_nonce() -> crate::Result<u64> {
  let nonce = CURR_NONCE.fetch_add(1, Ordering::Relaxed);
  let now_ms: u64 = Instant::now_timestamp(0)?.as_millis().try_into().map_err(wtx::Error::from)?;
  if nonce.wrapping_add(300_000) < now_ms {
    CURR_NONCE.fetch_max(now_ms.wrapping_add(1), Ordering::Relaxed);
    return Ok(now_ms);
  }
  Ok(nonce)
}

pub(crate) fn serialize_hex<S, T>(val: T, s: S) -> Result<S::Ok, S::Error>
where
  T: LowerHex,
  S: Serializer,
{
  s.collect_str(&format_args!("0x{val:x}"))
}

pub(crate) fn serialize_sig<S>(sig: &Signature, s: S) -> Result<S::Ok, S::Error>
where
  S: Serializer,
{
  let mut state = s.serialize_struct("Signature", 3)?;
  state.serialize_field("r", &sig.r)?;
  state.serialize_field("s", &sig.s)?;
  state.serialize_field("v", &u64::from(sig.v()))?;
  state.end()
}

pub(crate) fn sign_l1_action(
  buffer: &mut Vector<u8>,
  connection_id: [u8; 32],
  is_mainnet: bool,
  wallet: &k256::ecdsa::SigningKey,
) -> crate::Result<Signature> {
  let source = if is_mainnet { b"a" } else { b"b" };
  let payload = Agent { source, connection_id: ArrayWrapper(connection_id) };
  sign_typed_data(buffer, &payload, wallet)
}

pub(crate) fn sign_typed_data<T>(
  buffer: &mut Vector<u8>,
  payload: &T,
  wallet: &k256::ecdsa::SigningKey,
) -> crate::Result<Signature>
where
  T: Eip712,
{
  Ok(wallet.sign_prehash_recoverable(&payload.eip712_signing_hash(buffer)?)?.into())
}

#[cfg(test)]
mod tests {
  use crate::exchange::hyperliquid::misc::sign_l1_action;
  use k256::ecdsa::SigningKey;
  use wtx::collection::Vector;

  #[test]
  fn sign_l1_action_has_correct_output() {
    let wallet = wallet();
    let mut buffer = Vector::new();
    let connection_id = [
      222, 108, 64, 55, 121, 138, 68, 52, 202, 3, 205, 5, 240, 14, 59, 128, 49, 38, 34, 19, 117,
      205, 30, 126, 170, 175, 4, 23, 104, 190, 6, 235,
    ];

    {
      let expected = [
        250, 138, 65, 246, 163, 250, 114, 130, 6, 223, 128, 128, 26, 131, 188, 191, 186, 176, 134,
        73, 205, 52, 217, 192, 191, 186, 124, 123, 47, 153, 52, 15, 83, 160, 2, 38, 96, 69, 103,
        185, 138, 20, 146, 128, 49, 144, 214, 90, 32, 29, 104, 5, 229, 131, 27, 112, 68, 241, 127,
        213, 48, 174, 199, 132, 28,
      ];
      let sig = sign_l1_action(&mut buffer, connection_id, true, &wallet).unwrap();
      assert_eq!(sig.r.to_be_bytes(), &expected[..32]);
      assert_eq!(sig.s.to_be_bytes(), &expected[32..64]);
      assert_eq!(sig.v(), expected[64]);
    }
    {
      let expected = [
        23, 19, 192, 252, 102, 27, 121, 42, 80, 232, 255, 221, 89, 182, 55, 177, 237, 23, 45, 154,
        58, 164, 216, 1, 217, 216, 134, 70, 113, 15, 183, 75, 51, 149, 159, 77, 7, 90, 124, 203,
        236, 159, 35, 116, 166, 218, 33, 255, 164, 68, 141, 88, 208, 65, 58, 13, 51, 87, 117, 246,
        128, 168, 129, 67, 28,
      ];
      let sig = sign_l1_action(&mut buffer, connection_id, false, &wallet).unwrap();
      assert_eq!(sig.r.to_be_bytes(), &expected[..32]);
      assert_eq!(sig.s.to_be_bytes(), &expected[32..64]);
      assert_eq!(sig.v(), expected[64])
    }
  }

  fn wallet() -> SigningKey {
    let array = [
      233, 8, 248, 109, 187, 77, 85, 172, 135, 99, 120, 86, 90, 175, 234, 188, 24, 127, 102, 144,
      240, 70, 69, 147, 151, 177, 125, 155, 154, 25, 104, 142,
    ];
    SigningKey::from_bytes((&array).into()).unwrap()
  }
}
