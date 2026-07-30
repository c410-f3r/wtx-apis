use wtx::misc::ArithmeticError;

use crate::misc::U256;
use core::ops::Shr as _;

/// Estimates the liquidity for a position based on a range of ticks and token amounts.
#[inline]
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
#[inline]
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
  let mut ratio: u128 =
    if (tick & 1) != 0 { 18_445_821_805_675_392_311 } else { 18_446_744_073_709_551_616 };
  if (tick & 2) != 0 {
    ratio = ratio.wrapping_mul(18_444_899_583_751_176_498) >> 64;
  }
  if (tick & 4) != 0 {
    ratio = ratio.wrapping_mul(18_443_055_278_223_354_162) >> 64;
  }
  if (tick & 8) != 0 {
    ratio = ratio.wrapping_mul(18_439_367_220_385_604_838) >> 64;
  }
  if (tick & 16) != 0 {
    ratio = ratio.wrapping_mul(18_431_993_317_065_449_817) >> 64;
  }
  if (tick & 32) != 0 {
    ratio = ratio.wrapping_mul(18_417_254_355_718_160_513) >> 64;
  }
  if (tick & 64) != 0 {
    ratio = ratio.wrapping_mul(18_387_811_781_193_591_352) >> 64;
  }
  if (tick & 128) != 0 {
    ratio = ratio.wrapping_mul(18_329_067_761_203_520_168) >> 64;
  }
  if (tick & 256) != 0 {
    ratio = ratio.wrapping_mul(18_212_142_134_806_087_854) >> 64;
  }
  if (tick & 512) != 0 {
    ratio = ratio.wrapping_mul(17_980_523_815_641_551_639) >> 64;
  }
  if (tick & 1024) != 0 {
    ratio = ratio.wrapping_mul(17_526_086_738_831_147_013) >> 64;
  }
  if (tick & 2048) != 0 {
    ratio = ratio.wrapping_mul(16_651_378_430_235_024_244) >> 64;
  }
  if (tick & 4096) != 0 {
    ratio = ratio.wrapping_mul(15_030_750_278_693_429_944) >> 64;
  }
  if (tick & 8192) != 0 {
    ratio = ratio.wrapping_mul(12_247_334_978_882_834_399) >> 64;
  }
  if (tick & 16384) != 0 {
    ratio = ratio.wrapping_mul(8_131_365_268_884_726_200) >> 64;
  }
  if (tick & 32768) != 0 {
    ratio = ratio.wrapping_mul(3_584_323_654_723_342_297) >> 64;
  }
  if (tick & 65536) != 0 {
    ratio = ratio.wrapping_mul(696_457_651_847_595_233) >> 64;
  }
  if (tick & 131_072) != 0 {
    ratio = ratio.wrapping_mul(26_294_789_957_452_057) >> 64;
  }
  if (tick & 262_144) != 0 {
    ratio = ratio.wrapping_mul(37_481_735_321_082) >> 64;
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
  let mut ratio: u128 = if (tick & 1) != 0 {
    79_232_123_823_359_799_118_286_999_567
  } else {
    79_228_162_514_264_337_593_543_950_336
  };
  if (tick & 2) != 0 {
    ratio = mul_shift_96(ratio, 79_236_085_330_515_764_027_303_304_731);
  }
  if (tick & 4) != 0 {
    ratio = mul_shift_96(ratio, 79_244_008_939_048_815_603_706_035_061);
  }
  if (tick & 8) != 0 {
    ratio = mul_shift_96(ratio, 79_259_858_533_276_714_757_314_932_305);
  }
  if (tick & 16) != 0 {
    ratio = mul_shift_96(ratio, 79_291_567_232_598_584_799_939_703_904);
  }
  if (tick & 32) != 0 {
    ratio = mul_shift_96(ratio, 79_355_022_692_464_371_645_785_046_466);
  }
  if (tick & 64) != 0 {
    ratio = mul_shift_96(ratio, 79_482_085_999_252_804_386_437_311_141);
  }
  if (tick & 128) != 0 {
    ratio = mul_shift_96(ratio, 79_736_823_300_114_093_921_829_183_326);
  }
  if (tick & 256) != 0 {
    ratio = mul_shift_96(ratio, 80_248_749_790_819_932_309_965_073_892);
  }
  if (tick & 512) != 0 {
    ratio = mul_shift_96(ratio, 81_282_483_887_344_747_381_513_967_011);
  }
  if (tick & 1024) != 0 {
    ratio = mul_shift_96(ratio, 83_390_072_131_320_151_908_154_831_281);
  }
  if (tick & 2048) != 0 {
    ratio = mul_shift_96(ratio, 87_770_609_709_833_776_024_991_924_138);
  }
  if (tick & 4096) != 0 {
    ratio = mul_shift_96(ratio, 97_234_110_755_111_693_312_479_820_773);
  }
  if (tick & 8192) != 0 {
    ratio = mul_shift_96(ratio, 119_332_217_159_966_728_226_237_229_890);
  }
  if (tick & 16384) != 0 {
    ratio = mul_shift_96(ratio, 179_736_315_981_702_064_433_883_588_727);
  }
  if (tick & 32768) != 0 {
    ratio = mul_shift_96(ratio, 407_748_233_172_238_350_107_850_275_304);
  }
  if (tick & 65536) != 0 {
    ratio = mul_shift_96(ratio, 2_098_478_828_474_011_932_436_660_412_517);
  }
  if (tick & 131_072) != 0 {
    ratio = mul_shift_96(ratio, 55_581_415_166_113_811_149_459_800_483_533);
  }
  if (tick & 262_144) != 0 {
    ratio = mul_shift_96(ratio, 38_992_368_544_603_139_932_233_054_999_993_551);
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

#[expect(clippy::arithmetic_side_effects, reason = "64 does not extrapolate 256")]
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

#[expect(clippy::arithmetic_side_effects, reason = "64 does not extrapolate 256")]
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
