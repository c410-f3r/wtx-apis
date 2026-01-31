use crate::exchange::hyperliquid::{BuilderInfo, Cloid};
use rust_decimal::Decimal;

#[derive(Debug, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub enum OrderGrouping {
  Na,
  NormalTlsl,
  PositionTlsp,
}

/// Time in force for a limit order.
#[derive(Debug, serde::Deserialize, serde::Serialize)]
pub enum OrderTif {
  /// Good 'til canceled.
  #[serde(rename = "Gtc")]
  Gtc,
  /// Immediate or Cancel.
  #[serde(rename = "Ioc")]
  Ioc,
}

/// Take profit or stop loss for a trigger order.
#[derive(Debug, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub enum OrderTpsl {
  /// Take profit
  Tp,
  /// Stop loss
  Sl,
}

/// The type of order, which is either a limit or trigger order.
#[derive(Debug, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub enum OrderTy {
  /// See [`OrderLimitParams`].
  Limit(OrderLimitParams),
  /// See [`OrderTriggerParams`].
  Trigger(OrderTriggerParams),
}

#[derive(Debug, serde::Deserialize, serde::Serialize)]
pub struct ModifyReq {
  /// Order ID
  pub oid: u64,
  /// See [`OrderReq`].
  pub order: OrderReq,
}

/// Spot or perp order
#[derive(Debug, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct OrderReq {
  /// The asset's identifier.
  #[serde(rename = "a", alias = "asset")]
  pub asset: u32,
  /// Whether the order is a buy or a sell.
  #[serde(rename = "b", alias = "isBuy")]
  pub is_buy: bool,
  /// The limit price for the order.
  #[serde(rename = "p", alias = "limitPx")]
  pub limit_price: Decimal,
  /// The size of the order.
  #[serde(rename = "s", alias = "sz")]
  pub size: Decimal,
  /// Whether the order is reduce-only.
  #[serde(rename = "r", alias = "reduceOnly", default)]
  pub reduce_only: bool,
  /// See [`OrderTy`].
  #[serde(rename = "t", alias = "orderType")]
  pub ty: OrderTy,
  /// Client Order ID
  #[serde(rename = "c", alias = "cloid", skip_serializing_if = "Option::is_none")]
  pub cloid: Option<Cloid>,
}

/// Parameters of a limit order.
#[derive(Debug, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct OrderLimitParams {
  /// See [`OrderTif`].
  pub tif: OrderTif,
}

/// Parameters of a trigger order.
#[derive(Debug, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct OrderTriggerParams {
  /// Whether the trigger order should be a market order.
  pub is_market: bool,
  /// /// The price at which the order is triggered.
  pub trigger_px: Decimal,
  /// See [`OrderTpsl`].
  pub tpsl: OrderTpsl,
}

#[derive(Debug, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct BulkModify<'any> {
  /// See [`ModifyReq`].
  pub modifies: &'any [ModifyReq],
}

#[derive(Debug, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct BulkOrder<'any> {
  /// See [`Order`].
  pub orders: &'any [OrderReq],
  /// See [`OrderGrouping`].
  pub grouping: OrderGrouping,
  /// See [`BuilderInfo`].
  #[serde(skip_serializing_if = "Option::is_none")]
  pub builder: Option<BuilderInfo>,
}

#[cfg(test)]
mod tests {
  use crate::exchange::hyperliquid::{
    BulkOrder, OrderGrouping, OrderLimitParams, OrderReq, OrderTif, OrderTpsl, OrderTriggerParams,
    OrderTy, action::Action, misc::sign_l1_action,
  };
  use wtx::{collection::Vector, de::decode_hex};

  #[test]
  fn sign_order_limit() {
    let wallet = wallet();
    let mut buffer = Vector::new();
    let action = Action::Order(BulkOrder {
      orders: &[OrderReq {
        asset: 1,
        is_buy: true,
        limit_price: "2000.0".try_into().unwrap(),
        size: "3.5".try_into().unwrap(),
        reduce_only: false,
        ty: OrderTy::Limit(OrderLimitParams { tif: OrderTif::Ioc }),
        cloid: None,
      }],
      grouping: OrderGrouping::Na,
      builder: None,
    });
    let connection_id = action.hash(&mut buffer, 1583838, None).unwrap();
    assert_eq!(
      sign_l1_action(&mut buffer, connection_id, true, &wallet).unwrap().all_bytes(),
      decode_hex(
        b"0x77957e58e70f43b6b68581f2dc42011fc384538a2e5b7bf42d5b936f19fbb67360721a8598727230f67080efee48c812a6a4442013fd3b0eed509171bef9f23f1c",
        &mut [0; 65]
      ).unwrap()
    );
    assert_eq!(
      sign_l1_action(&mut buffer, connection_id, false, &wallet).unwrap().all_bytes(),
      decode_hex(
        b"0xcd0925372ff1ed499e54883e9a6205ecfadec748f80ec463fe2f84f1209648776377961965cb7b12414186b1ea291e95fd512722427efcbcfb3b0b2bcd4d79d01c",
        &mut [0; 65]
      ).unwrap()
    );
  }

  #[test]
  fn sign_order_limit_with_cloid() {
    let wallet = wallet();
    let mut buffer = Vector::new();
    let action = Action::Order(BulkOrder {
      orders: &[OrderReq {
        asset: 1,
        is_buy: true,
        limit_price: "2000.0".try_into().unwrap(),
        size: "3.5".try_into().unwrap(),
        reduce_only: false,
        ty: OrderTy::Limit(OrderLimitParams { tif: OrderTif::Ioc }),
        cloid: Some("0x1e60610f0b3d420597c88c1fed2ad5ee".try_into().unwrap()),
      }],
      grouping: OrderGrouping::Na,
      builder: None,
    });
    let connection_id = action.hash(&mut buffer, 1583838, None).unwrap();
    assert_eq!(
      sign_l1_action(&mut buffer, connection_id, true, &wallet).unwrap().all_bytes(),
      decode_hex(
        b"0xd3e894092eb27098077145714630a77bbe3836120ee29df7d935d8510b03a08f456de5ec1be82aa65fc6ecda9ef928b0445e212517a98858cfaa251c4cd7552b1c",
        &mut [0; 65]
      ).unwrap()
    );
    assert_eq!(
      sign_l1_action(&mut buffer, connection_id, false, &wallet).unwrap().all_bytes(),
      decode_hex(
        b"0x3768349dbb22a7fd770fc9fc50c7b5124a7da342ea579b309f58002ceae49b4357badc7909770919c45d850aabb08474ff2b7b3204ae5b66d9f7375582981f111c",
        &mut [0; 65]
      ).unwrap()
    );
  }

  #[test]
  fn sign_order_trigger() {
    let params = [
      (
        OrderTpsl::Tp,
        b"0xb91e5011dff15e4b4a40753730bda44972132e7b75641f3cac58b66159534a170d422ee1ac3c7a7a2e11e298108a2d6b8da8612caceaeeb3e571de3b2dfda9e41b",
        b"0x6df38b609904d0d4439884756b8f366f22b3a081801dbdd23f279094a2299fac6424cb0cdc48c3706aeaa368f81959e91059205403d3afd23a55983f710aee871b",
      ),
      (
        OrderTpsl::Sl,
        b"0x8456d2ace666fce1bee1084b00e9620fb20e810368841e9d4dd80eb29014611a0843416e51b1529c22dd2fc28f7ff8f6443875635c72011f60b62cbb8ce90e2d1c",
        b"0xeb5bdb52297c1d19da45458758bd569dcb24c07e5c7bd52cf76600fd92fdd8213e661e21899c985421ec018a9ee7f3790e7b7d723a9932b7b5adcd7def5354601c",
      ),
    ];

    let wallet = wallet();
    let mut buffer = Vector::new();
    for (tpsl, mainnet, testnet) in params {
      let action = Action::Order(BulkOrder {
        orders: &[OrderReq {
          asset: 1,
          is_buy: true,
          limit_price: "2000.0".try_into().unwrap(),
          size: "3.5".try_into().unwrap(),
          reduce_only: false,
          ty: OrderTy::Trigger(OrderTriggerParams {
            is_market: true,
            trigger_px: "2000.0".try_into().unwrap(),
            tpsl,
          }),
          cloid: None,
        }],
        grouping: OrderGrouping::Na,
        builder: None,
      });
      let connection_id = action.hash(&mut buffer, 1583838, None).unwrap();
      assert_eq!(
        sign_l1_action(&mut buffer, connection_id, true, &wallet).unwrap().all_bytes(),
        decode_hex(mainnet, &mut [0; 65]).unwrap()
      );
      assert_eq!(
        sign_l1_action(&mut buffer, connection_id, false, &wallet).unwrap().all_bytes(),
        decode_hex(testnet, &mut [0; 65]).unwrap()
      );
    }
  }

  fn wallet() -> k256::ecdsa::SigningKey {
    k256::ecdsa::SigningKey::from_bytes(
      &([
        233, 8, 248, 109, 187, 77, 85, 172, 135, 99, 120, 86, 90, 175, 234, 188, 24, 127, 102, 144,
        240, 70, 69, 147, 151, 177, 125, 155, 154, 25, 104, 142,
      ])
      .into(),
    )
    .unwrap()
  }
}
