use wtx::misc::ArithmeticError;

use crate::misc::U256;
use core::ops::Shr;

/// Estimates the liquidity for a position based on a range of ticks and token amounts.
#[track_caller]
pub fn estimate_liquidity_from_token_amounts(
  [lower_tick, curr_tick, upper_tick]: [i32; 3],
  [token_a, token_b]: [u64; 2],
) -> crate::Result<u128> {
  let curr_sqrt_price = tick_idx_to_sqrt_price_x64(curr_tick);
  let lower_sqrt_price = tick_idx_to_sqrt_price_x64(lower_tick);
  let upper_sqrt_price = tick_idx_to_sqrt_price_x64(upper_tick);
  Ok(if curr_sqrt_price >= upper_sqrt_price {
    estimate_liquidity_for_token_b(upper_sqrt_price, lower_sqrt_price, token_b)?
  } else if curr_sqrt_price < lower_sqrt_price {
    estimate_liquidity_for_token_a(lower_sqrt_price, upper_sqrt_price, token_a)?
  } else {
    let amount_a = estimate_liquidity_for_token_a(curr_sqrt_price, upper_sqrt_price, token_a)?;
    let amount_b = estimate_liquidity_for_token_b(curr_sqrt_price, lower_sqrt_price, token_b)?;
    amount_a.min(amount_b)
  })
}

/// Estimates the expected token amounts for a position based on a range of ticks and liquidity.
pub fn estimate_token_amounts_from_liquidity(
  [lower_tick, curr_tick, upper_tick]: [i32; 3],
  liquidity: u128,
) -> crate::Result<[u128; 2]> {
  if liquidity == 0 {
    return Err(crate::Error::SolanaLiquidityMustBeGreaterThanZero);
  }
  let curr_sqrt_price = tick_idx_to_sqrt_price_x64(curr_tick);
  let lower_sqrt_price = tick_idx_to_sqrt_price_x64(lower_tick);
  let upper_sqrt_price = tick_idx_to_sqrt_price_x64(upper_tick);
  let position_status = position_status(curr_sqrt_price, lower_sqrt_price, upper_sqrt_price);
  let mut token0 = 0;
  let mut token1 = 0;
  match position_status {
    PositionStatus::BelowRange => {
      token0 = token_a_from_liquidity(liquidity, lower_sqrt_price, upper_sqrt_price)?;
    }
    PositionStatus::InRange => {
      token0 = token_a_from_liquidity(liquidity, curr_sqrt_price, upper_sqrt_price)?;
      token1 = token_b_from_liquidity(liquidity, lower_sqrt_price, curr_sqrt_price)?;
    }
    PositionStatus::AboveRange => {
      token1 = token_b_from_liquidity(liquidity, lower_sqrt_price, upper_sqrt_price)?;
    }
  }
  Ok([token0, token1])
}

#[derive(Debug)]
enum PositionStatus {
  BelowRange,
  InRange,
  AboveRange,
}

fn estimate_liquidity_for_token_a(
  sqrt_price0: u128,
  sqrt_price1: u128,
  token_amount: u64,
) -> crate::Result<u128> {
  let lower_sqrt_price_x64 = sqrt_price0.min(sqrt_price1);
  let upper_sqrt_price_x64 = sqrt_price0.max(sqrt_price1);
  let num_x64 = U256::from(token_amount)
    .mul(U256::from(upper_sqrt_price_x64))
    .and_then(|el| el.mul(U256::from(lower_sqrt_price_x64)))
    .unwrap_or(U256::MAX);
  let num = from_x64(num_x64)?;
  let dem = upper_sqrt_price_x64.wrapping_sub(lower_sqrt_price_x64);
  Ok(num.checked_div(dem).ok_or(wtx::Error::from(ArithmeticError::DivOverflow))?)
}

fn estimate_liquidity_for_token_b(
  sqrt_price0: u128,
  sqrt_price1: u128,
  token_amount: u64,
) -> crate::Result<u128> {
  let lower_sqrt_price_x64 = sqrt_price0.min(sqrt_price1);
  let upper_sqrt_price_x64 = sqrt_price0.max(sqrt_price1);
  let delta = upper_sqrt_price_x64.wrapping_sub(lower_sqrt_price_x64);
  Ok(
    to_x64(token_amount)
      .checked_div(delta)
      .ok_or(wtx::Error::from(ArithmeticError::DivOverflow))?,
  )
}

fn from_x64(num: U256) -> crate::Result<u128> {
  num.div(U256::from(2u128.pow(64)))?.try_into()
}

fn order_sqrt_price(sqrt_price0_x64: u128, sqrt_price1_x64: u128) -> [u128; 2] {
  if sqrt_price0_x64 < sqrt_price1_x64 {
    [sqrt_price0_x64, sqrt_price1_x64]
  } else {
    [sqrt_price1_x64, sqrt_price0_x64]
  }
}

fn position_status(
  sqrt_price_x64: u128,
  sqrt_price_lower_x64: u128,
  sqrt_price_upper_x64: u128,
) -> PositionStatus {
  if sqrt_price_x64 <= sqrt_price_lower_x64 {
    PositionStatus::BelowRange
  } else if sqrt_price_x64 >= sqrt_price_upper_x64 {
    PositionStatus::AboveRange
  } else {
    PositionStatus::InRange
  }
}

fn tick_idx_to_sqrt_price_x64_negative(tick_idx: i32) -> u128 {
  let tick = tick_idx.abs();
  let mut ratio: u128 = if (tick & 1) != 0 { 18445821805675392311 } else { 18446744073709551616 };
  if (tick & 2) != 0 {
    ratio = ratio.wrapping_mul(18444899583751176498) >> 64;
  }
  if (tick & 4) != 0 {
    ratio = ratio.wrapping_mul(18443055278223354162) >> 64;
  }
  if (tick & 8) != 0 {
    ratio = ratio.wrapping_mul(18439367220385604838) >> 64;
  }
  if (tick & 16) != 0 {
    ratio = ratio.wrapping_mul(18431993317065449817) >> 64;
  }
  if (tick & 32) != 0 {
    ratio = ratio.wrapping_mul(18417254355718160513) >> 64;
  }
  if (tick & 64) != 0 {
    ratio = ratio.wrapping_mul(18387811781193591352) >> 64;
  }
  if (tick & 128) != 0 {
    ratio = ratio.wrapping_mul(18329067761203520168) >> 64;
  }
  if (tick & 256) != 0 {
    ratio = ratio.wrapping_mul(18212142134806087854) >> 64;
  }
  if (tick & 512) != 0 {
    ratio = ratio.wrapping_mul(17980523815641551639) >> 64;
  }
  if (tick & 1024) != 0 {
    ratio = ratio.wrapping_mul(17526086738831147013) >> 64;
  }
  if (tick & 2048) != 0 {
    ratio = ratio.wrapping_mul(16651378430235024244) >> 64;
  }
  if (tick & 4096) != 0 {
    ratio = ratio.wrapping_mul(15030750278693429944) >> 64;
  }
  if (tick & 8192) != 0 {
    ratio = ratio.wrapping_mul(12247334978882834399) >> 64;
  }
  if (tick & 16384) != 0 {
    ratio = ratio.wrapping_mul(8131365268884726200) >> 64;
  }
  if (tick & 32768) != 0 {
    ratio = ratio.wrapping_mul(3584323654723342297) >> 64;
  }
  if (tick & 65536) != 0 {
    ratio = ratio.wrapping_mul(696457651847595233) >> 64;
  }
  if (tick & 131072) != 0 {
    ratio = ratio.wrapping_mul(26294789957452057) >> 64;
  }
  if (tick & 262144) != 0 {
    ratio = ratio.wrapping_mul(37481735321082) >> 64;
  }
  ratio
}

// Performs the exponential conversion with Q64.64 precision
fn tick_idx_to_sqrt_price_x64_positive(tick_idx: i32) -> u128 {
  fn mul_shift_96(lhs: u128, rhs: u128) -> u128 {
    U256::from(lhs)
      .mul(U256::from(rhs))
      .ok()
      .and_then(|el| el.shr(96).try_into().ok())
      .unwrap_or_default()
  }

  let tick = tick_idx.abs();
  let mut ratio: u128 =
    if (tick & 1) != 0 { 79232123823359799118286999567 } else { 79228162514264337593543950336 };
  if (tick & 2) != 0 {
    ratio = mul_shift_96(ratio, 79236085330515764027303304731);
  }
  if (tick & 4) != 0 {
    ratio = mul_shift_96(ratio, 79244008939048815603706035061);
  }
  if (tick & 8) != 0 {
    ratio = mul_shift_96(ratio, 79259858533276714757314932305);
  }
  if (tick & 16) != 0 {
    ratio = mul_shift_96(ratio, 79291567232598584799939703904);
  }
  if (tick & 32) != 0 {
    ratio = mul_shift_96(ratio, 79355022692464371645785046466);
  }
  if (tick & 64) != 0 {
    ratio = mul_shift_96(ratio, 79482085999252804386437311141);
  }
  if (tick & 128) != 0 {
    ratio = mul_shift_96(ratio, 79736823300114093921829183326);
  }
  if (tick & 256) != 0 {
    ratio = mul_shift_96(ratio, 80248749790819932309965073892);
  }
  if (tick & 512) != 0 {
    ratio = mul_shift_96(ratio, 81282483887344747381513967011);
  }
  if (tick & 1024) != 0 {
    ratio = mul_shift_96(ratio, 83390072131320151908154831281);
  }
  if (tick & 2048) != 0 {
    ratio = mul_shift_96(ratio, 87770609709833776024991924138);
  }
  if (tick & 4096) != 0 {
    ratio = mul_shift_96(ratio, 97234110755111693312479820773);
  }
  if (tick & 8192) != 0 {
    ratio = mul_shift_96(ratio, 119332217159966728226237229890);
  }
  if (tick & 16384) != 0 {
    ratio = mul_shift_96(ratio, 179736315981702064433883588727);
  }
  if (tick & 32768) != 0 {
    ratio = mul_shift_96(ratio, 407748233172238350107850275304);
  }
  if (tick & 65536) != 0 {
    ratio = mul_shift_96(ratio, 2098478828474011932436660412517);
  }
  if (tick & 131072) != 0 {
    ratio = mul_shift_96(ratio, 55581415166113811149459800483533);
  }
  if (tick & 262144) != 0 {
    ratio = mul_shift_96(ratio, 38992368544603139932233054999993551);
  }
  ratio >> 32
}

// Performs the exponential conversion with Q64.64 precision
fn tick_idx_to_sqrt_price_x64(tick_idx: i32) -> u128 {
  if tick_idx > 0 {
    tick_idx_to_sqrt_price_x64_positive(tick_idx)
  } else {
    tick_idx_to_sqrt_price_x64_negative(tick_idx)
  }
}

fn to_x64(num: u64) -> u128 {
  u128::from(num) << 64
}

fn token_a_from_liquidity(
  liquidity: u128,
  sqrt_price0_x64: u128,
  sqrt_price1_x64: u128,
) -> crate::Result<u128> {
  let [lower, upper] = order_sqrt_price(sqrt_price0_x64, sqrt_price1_x64);
  let mut num = U256::from(upper.saturating_sub(lower));
  num = U256::from(liquidity).mul(num)? << 64;
  let den = U256::from(upper).mul(U256::from(lower))?;
  u128::try_from(num.div(den)?)
}

fn token_b_from_liquidity(
  liquidity: u128,
  sqrt_price0_x64: u128,
  sqrt_price1_x64: u128,
) -> crate::Result<u128> {
  let [lower, upper] = order_sqrt_price(sqrt_price0_x64, sqrt_price1_x64);
  let rslt = U256::from(liquidity).mul(U256::from(upper.saturating_sub(lower)))?;
  u128::try_from(rslt >> 64)
}

#[cfg(test)]
mod test {
  use crate::blockchain::solana::program::orca_v3::{
    estimate_token_amounts_from_liquidity,
    math::{estimate_liquidity_from_token_amounts, tick_idx_to_sqrt_price_x64_positive},
  };

  #[test]
  fn tick_idx_to_sqrt_price_x64_positive_has_correct_output() {
    assert_eq!(tick_idx_to_sqrt_price_x64_positive(67372), 535563499758033622197);
  }

  // SOL-ORCA with 9-6 decimals
  #[test]
  fn liquidity_estimation_have_correct_output() {
    let liquidity = 688807747;
    // 0.046855431 SOL - 6.681408 ORCA
    assert_eq!(
      estimate_liquidity_from_token_amounts([-30336, -30191, -29888], [46855431, 6681408]).unwrap(),
      liquidity
    );
    // 0.04685543 SOL - 1.099726 ORCA
    assert_eq!(
      estimate_token_amounts_from_liquidity([-30336, -30191, -29888], liquidity).unwrap(),
      [46855430, 1099726]
    )
  }
}
