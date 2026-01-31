use rust_decimal::Decimal;
use wtx::collection::ArrayStringU8;

pub struct TradingFee {
  pub builder_fee_rate: Decimal,
  pub market: ArrayStringU8<11>,
  pub maker_fee_rate: Decimal,
  pub taker_fee_rate: Decimal,
}

impl TradingFee {
  pub const DEFAULT_FEES: TradingFee = TradingFee {
    builder_fee_rate: Decimal::ZERO,
    market: ArrayStringU8::new(),
    maker_fee_rate: Decimal::from_parts(2, 0, 0, false, 4),
    taker_fee_rate: Decimal::from_parts(5, 0, 0, false, 4),
  };
}
