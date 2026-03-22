use crate::AssetString;
use rust_decimal::Decimal;
use wtx::collection::Vector;

/// Account information response from the exchange API
#[derive(Debug, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Account {
  /// Fee tier level for the account
  pub fee_tier: u32,
  /// Whether the account can execute trades
  pub can_trade: bool,
  /// Whether the account can make deposits
  pub can_deposit: bool,
  /// Whether the account can make withdrawals
  pub can_withdraw: bool,
  /// Whether the account can burn assets
  pub can_burn_asset: bool,
  /// Last update timestamp in milliseconds
  pub update_time: u64,
  /// List of asset balances in the account
  pub balances: Vector<BalanceHttp>,
}

/// Individual asset balance information
#[derive(Debug, serde::Deserialize, serde::Serialize)]
pub struct BalanceHttp {
  /// Asset symbol (e.g., "BTC", "LTC")
  pub asset: AssetString,
  /// Available balance for trading/withdrawal
  pub free: Decimal,
  /// Balance locked in open orders
  pub locked: Decimal,
}

#[wtx::pkg(data_format(json), id(crate::exchange::aster::AsterId), transport(http))]
pub(crate) mod pkg {
  use crate::exchange::aster::{Account, Aster, CexSignParams, HttpPkgsAux};
  use wtx::{client_api_framework::pkg::PkgsAux, misc::LeaseMut};

  #[pkg::aux]
  impl<A, DRSR> HttpPkgsAux<A, DRSR>
  where
    A: LeaseMut<Aster>,
  {
    #[pkg::aux_data]
    fn account_data(&mut self, params: Option<&CexSignParams>) -> crate::Result<()> {
      let PkgsAux { api, bytes_buffer, encode_data, tp, .. } = &mut self.0;
      api.lease().auth_req::<false, _>(
        bytes_buffer,
        encode_data,
        params,
        if api.lease().is_dex {
          format_args!("/api/v3/account")
        } else {
          format_args!("/api/v1/account")
        },
        None,
        tp,
      )
    }
  }

  #[pkg::req_data]
  pub type AccountReq = ();

  #[pkg::res_data]
  pub type AccountRes = Account;
}
