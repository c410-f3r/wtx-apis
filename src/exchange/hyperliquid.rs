mod action;
mod agent;
mod builder_info;
mod cancel;
mod chain;
mod info_req;
//#[cfg(all(test, feature = "_integration-tests"))]
//mod integration_tests;
mod misc;
mod order;
mod order_info;
mod payload;
mod pkg;
mod signature;
mod spot_send;
mod spot_user;
mod usd_send;
mod web_socket;

pub use builder_info::BuilderInfo;
pub use cancel::*;
pub use chain::Chain;
pub use order::*;
pub use order_info::*;
pub use payload::ExchangeResponse;
pub use spot_send::*;
pub use spot_user::*;
pub use web_socket::*;
use wtx::client_api_framework::Api;

const SIGNATURE_CHAIN_ID: u64 = 421614;

/// HTTP Mainnet URI
pub static HTTP_MAINNET_URI: &str = "https://api.hyperliquid.xyz";
/// HTTP Testnet URI
pub static HTTP_TESTNET_URI: &str = "https://api.hyperliquid-testnet.xyz";
/// WebSocket Mainnet URI
pub static WS_MAINNET_URI: &str = "wss://api.hyperliquid.xyz/ws";
/// WebSocket Testnet URI
pub static WS_TESTNET_URI: &str = "wss://api.hyperliquid-testnet.xyz/ws";

/// Client Order ID
pub type Cloid = wtx::collection::ArrayStringU8<34>;

#[derive(Debug)]
#[doc = _generic_api_doc!()]
#[wtx::api(error(crate::Error), pkgs_aux(PkgsAux), transport(http, ws))]
pub struct Hyperliquid {
  is_mainnet: bool,
}

impl Hyperliquid {
  /// Creates a new instance with the given credentials.
  pub const fn new(is_mainnet: bool) -> Self {
    Self { is_mainnet }
  }
}

impl Api for Hyperliquid {
  type Error = crate::Error;
  type Id = HyperliquidId;

  async fn before_sending(&mut self) -> Result<(), Self::Error> {
    Ok(())
  }
}

wtx::create_packages_aux_wrapper!();
