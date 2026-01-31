// https://github.com/x10xchange/stark-crypto-wrapper-py/blob/e2fd1902b126b4e91f0b4cfd490d673fe178785b/src/lib.rs#L146
// https://github.com/x10xchange/stark-crypto-wrapper-py/blob/e2fd1902b126b4e91f0b4cfd490d673fe178785b/python/fast_stark_crypto/lib.py#L37

use crate::exchange::extended::{Asset, PoseidonHasher, StarknetDomain};
use core::borrow::Borrow;
use rust_decimal::{Decimal, RoundingStrategy};
use sha3::{Digest, Keccak256};
use starknet_crypto::{ExtendedSignature, SignError, rfc6979_generate_k, sign};
use starknet_types_core::felt::Felt;

#[derive(Debug)]
pub struct StarkSignature {
  pub r: Felt,
  pub s: Felt,
  pub v: Felt,
}

#[derive(Debug)]
pub struct StarkAmount<A> {
  asset: A,
  value: i64,
}

impl<A> StarkAmount<A>
where
  A: Borrow<Asset>,
{
  pub fn from_user(value: Decimal, asset: A, rounding: RoundingStrategy) -> crate::Result<Self> {
    let converted_value = asset
      .borrow()
      .convert_human_readable_to_stark_quantity(value, rounding)
      .ok_or(crate::Error::ExtendedBadAssetValue)?;
    Ok(Self { value: converted_value, asset })
  }

  pub fn asset(&self) -> &A {
    &self.asset
  }

  pub fn value(&self) -> i64 {
    self.value
  }

  pub fn value_mut(&mut self) -> &mut i64 {
    &mut self.value
  }
}

pub fn cairo_short_string_to_felt(str: &str) -> Felt {
  if str.len() > 31 {
    panic!()
  }
  let mut buffer = [0u8; 32];
  let begin = 32usize.wrapping_sub(str.len());
  let bytes = str.as_bytes();
  buffer.get_mut(begin..).unwrap_or_default().copy_from_slice(bytes);
  Felt::from_bytes_be(&buffer)
}

pub fn hash_message(
  msg: Felt,
  public_key: Felt,
  stark_domain: &StarknetDomain<&str>,
) -> crate::Result<Felt> {
  // StarkNet Message
  const MSG: Felt = Felt::from_raw([
    257012186512350467,
    18446744073709551605,
    10480951322775611302,
    16156019428408348868,
  ]);

  let mut hasher = PoseidonHasher::new();
  hasher.update(MSG);
  hasher.update(stark_domain.hash());
  hasher.update(public_key);
  hasher.update(msg);
  Ok(hasher.finalize())
}

pub fn hash_order(
  amount_synthetic: StarkAmount<&Asset>,
  amount_collateral: StarkAmount<&Asset>,
  max_fee: StarkAmount<&Asset>,
  nonce: u32,
  position_id: u32,
  expiration_timestamp_secs: u64,
  public_key_hex: &str,
  starknet_domain: &StarknetDomain<&str>,
) -> crate::Result<Felt> {
  let base_asset_id = Felt::from_hex(&amount_synthetic.asset.settlement_external_id).unwrap();
  let quote_asset_id = Felt::from_hex(&amount_collateral.asset.settlement_external_id).unwrap();
  let user_key = Felt::from_hex(&public_key_hex).unwrap();
  let order = Order {
    position_id: PositionId { value: position_id },
    base_asset_id: AssetId { value: base_asset_id },
    base_amount: amount_synthetic.value,
    quote_asset_id: AssetId { value: quote_asset_id },
    quote_amount: amount_collateral.value,
    fee_asset_id: AssetId { value: quote_asset_id },
    fee_amount: max_fee.value.cast_unsigned(),
    expiration: Timestamp { seconds: expiration_timestamp_secs },
    salt: nonce.try_into().unwrap(),
  };
  Ok(hash_message(order.hash(), user_key, &starknet_domain)?)
}

pub fn rs_sign_message(secret_key_hex: &str, msg_hash_hex: &str) -> crate::Result<(Felt, Felt)> {
  let msg_hash = Felt::from_hex(msg_hash_hex).unwrap();
  let priv_key = Felt::from_hex(secret_key_hex).unwrap();
  let signature = sign_message(&msg_hash, &priv_key).unwrap();
  Ok((signature.r, signature.s))
}

pub fn sign_message(message: &Felt, private_key: &Felt) -> crate::Result<StarkSignature> {
  let signature = ecdsa_sign(private_key, &message)?;
  Ok(StarkSignature { r: signature.r, s: signature.s, v: signature.v })
}

struct AssetId {
  pub value: Felt,
}

struct Order {
  position_id: PositionId,
  base_asset_id: AssetId,
  base_amount: i64,
  quote_asset_id: AssetId,
  quote_amount: i64,
  fee_asset_id: AssetId,
  fee_amount: u64,
  expiration: Timestamp,
  salt: Felt,
}

impl Order {
  // "Order"(
  //   "position_id":"felt",
  //    "base_asset_id":"AssetId",
  //    "base_amount":"i64",
  //    "quote_asset_id":"AssetId",
  //    "quote_amount":"i64",
  //    "fee_asset_id":"AssetId",
  //    "fee_amount":"u64",
  //    "expiration":"Timestamp",
  //    "salt":"felt"
  //  )
  // "PositionId"("value":"u32")
  // "AssetId"("value":"felt")
  // "Timestamp"("seconds":"u64")
  pub const SELECTOR: Felt = Felt::from_raw([
    77858131504379635,
    11638252743578785094,
    15269766786444191933,
    4810005202040474528,
  ]);

  fn hash(&self) -> Felt {
    let mut hasher = PoseidonHasher::new();
    hasher.update(Self::SELECTOR);
    hasher.update(self.position_id.value.into());
    hasher.update(self.base_asset_id.value.into());
    hasher.update(self.base_amount.into());
    hasher.update(self.quote_asset_id.value.into());
    hasher.update(self.quote_amount.into());
    hasher.update(self.fee_asset_id.value.into());
    hasher.update(self.fee_amount.into());
    hasher.update(self.expiration.seconds.into());
    hasher.update(self.salt);
    hasher.finalize()
  }
}

struct PositionId {
  value: u32,
}

struct Timestamp {
  seconds: u64,
}

fn ecdsa_sign(private_key: &Felt, message_hash: &Felt) -> crate::Result<ExtendedSignature> {
  let mut seed = None;
  loop {
    let k = rfc6979_generate_k(message_hash, private_key, seed.as_ref());
    match sign(private_key, message_hash, &k) {
      Ok(sig) => {
        return Ok(sig);
      }
      Err(SignError::InvalidMessageHash) => panic!(),
      Err(SignError::InvalidK) => {
        // Bump seed and retry
        seed = match seed {
          Some(prev_seed) => Some(prev_seed + Felt::ONE),
          None => Some(Felt::ONE),
        };
      }
    }
  }
}

// https://docs.rs/starknet-ff/latest/starknet_ff/struct.FieldElement.html#method.into_mont
fn _starknet_keccak(data: &[u8]) -> Felt {
  let mut hasher = Keccak256::new();
  hasher.update(data);
  let mut hash: [u8; 32] = hasher.finalize().into();
  hash[0] &= 0b00000011;
  Felt::from_bytes_be(&hash)
}

#[cfg(test)]
mod tests {
  use crate::exchange::extended::{
    EndpointConfig, L2Config, Market, MarketStats, StarkAmount, StarknetDomain, TradingConfig,
    hash_message, hash_order,
    misc::{AssetId, Order, PositionId, Timestamp},
    rs_sign_message,
  };
  use rust_decimal::{Decimal, RoundingStrategy};
  use starknet_crypto::Felt;

  #[test]
  fn domain_select_hash() {
    assert_eq!(
      Felt::from_hex("0x1ff2f602e42168014d405a94f75e8a93d640751d71d16311266e140d8b0a210").unwrap(),
      StarknetDomain::<&str>::SELECTOR
    );
  }

  #[test]
  fn domain_hash() {
    assert_eq!(
      EndpointConfig::TESTNET_CONFIG.starknet_domain.hash(),
      Felt::from_dec_str(
        "2788850828067604540663615870177667078542240404906059806659101905868929188327",
      )
      .unwrap()
    );
  }

  #[test]
  fn order_hash() {
    let order = Order {
      position_id: PositionId { value: 1 },
      base_asset_id: AssetId { value: Felt::from_dec_str("2").unwrap() },
      base_amount: 3,
      quote_asset_id: AssetId { value: Felt::from_dec_str("4").unwrap() },
      quote_amount: 5,
      fee_asset_id: AssetId { value: Felt::from_dec_str("6").unwrap() },
      fee_amount: 7,
      expiration: Timestamp { seconds: 8 },
      salt: Felt::from_dec_str("9").unwrap(),
    };
    assert_eq!(
      order.hash(),
      Felt::from_dec_str(
        "1329353150252109345267997901008558234696410103652961347079636617692652241760",
      )
      .unwrap()
    );
  }

  #[test]
  fn hash_message_has_correct_results() {
    let order = Order {
      position_id: PositionId { value: 1 },
      base_asset_id: AssetId { value: Felt::from_dec_str("2").unwrap() },
      base_amount: 3,
      quote_asset_id: AssetId { value: Felt::from_dec_str("4").unwrap() },
      quote_amount: 5,
      fee_asset_id: AssetId { value: Felt::from_dec_str("6").unwrap() },
      fee_amount: 7,
      expiration: Timestamp { seconds: 8 },
      salt: Felt::from_dec_str("9").unwrap(),
    };
    let user_key = Felt::from_dec_str(
      "1528491859474308181214583355362479091084733880193869257167008343298409336538",
    )
    .unwrap();
    assert_eq!(
      hash_message(order.hash(), user_key, &EndpointConfig::TESTNET_CONFIG.starknet_domain)
        .unwrap(),
      Felt::from_dec_str(
        "2788960362996410178586013462192086205585543858281504820767681025777602529597",
      )
      .unwrap()
    );
  }

  #[test]
  fn hash_order_has_correct_results() {
    let pk_hex = "0x5d05989e9302dcebc74e241001e3e3ac3f4402ccf2f8e6f74b034b07ad6a904";
    let sk_hex = "0x123456";
    let market = Market {
      name: "BTC-USD".try_into().unwrap(),
      asset_name: "BTC".try_into().unwrap(),
      asset_precision: 5,
      collateral_asset_name: "USD".try_into().unwrap(),
      collateral_asset_precision: 6,
      active: true,
      market_stats: MarketStats::default(),
      trading_config: TradingConfig::default(),
      l2_config: L2Config {
        ty: "STARKX".try_into().unwrap(),
        collateral_id: "0x31857064564ed0ff978e687456963cba09c2c6985d8f9300a1de4962fafa054"
          .try_into()
          .unwrap(),
        collateral_resolution: 1000000,
        synthetic_id: "0x4254432d3600000000000000000000".try_into().unwrap(),
        synthetic_resolution: 1000000,
      },
    };
    let collateral_asset = market.collateral_asset();
    let synthetic_asset = market.synthetic_asset();
    let amount_collateral = StarkAmount::from_user(
      Decimal::try_from("-66").unwrap(),
      &collateral_asset,
      RoundingStrategy::AwayFromZero,
    )
    .unwrap();
    let amount_synthetic = StarkAmount::from_user(
      Decimal::try_from("100").unwrap(),
      &synthetic_asset,
      RoundingStrategy::AwayFromZero,
    )
    .unwrap();
    let max_fee = StarkAmount::from_user(
      Decimal::try_from("0.033").unwrap(),
      &collateral_asset,
      RoundingStrategy::AwayFromZero,
    )
    .unwrap();
    let hash = hash_order(
      amount_synthetic,
      amount_collateral,
      max_fee,
      1549931574,
      500098,
      1759884434,
      pk_hex,
      &EndpointConfig::TESTNET_CONFIG.starknet_domain,
    )
    .unwrap();
    assert_eq!(
      hash.to_hex_string(),
      "0x14beae80a9d1eef39c81d0f874d70646e842827c8efddc2307c9a75af2a244c"
    );
    let signature = rs_sign_message(sk_hex, &hash.to_hex_string()).unwrap();
    assert_eq!(
      (signature.0.to_hex_string(), signature.1.to_hex_string()),
      (
        "0x518acf0190b0118c338afdd0825a92fc7928471e801c834f385da997f53917b".into(),
        "0x686060febbc283a5e1795675862292b37b96dd8dcd3329bba1dc9159e58c86".into(),
      )
    );
  }

  #[test]
  fn selector_hash() {
    assert_eq!(
      Felt::from_hex("0x36da8d51815527cabfaa9c982f564c80fa7429616739306036f1f9b608dd112").unwrap(),
      Order::SELECTOR
    );
  }
}
