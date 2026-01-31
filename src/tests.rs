use std::sync::LazyLock;
use tokio::runtime::Runtime;

pub(crate) static _RUNTIME: LazyLock<Runtime> =
  LazyLock::new(|| tokio::runtime::Builder::new_current_thread().enable_all().build().unwrap());
pub(crate) static _VARS: LazyLock<_Vars> =
  LazyLock::new(|| wtx::misc::EnvVars::from_available().unwrap().finish());

#[derive(Debug, wtx::FromVars)]
pub(crate) struct _Vars {
  #[allow(unused, reason = "testnet key")]
  #[cfg(feature = "aster")]
  pub(crate) aster_api_key: alloc::string::String,
  #[allow(unused, reason = "testnet key")]
  #[cfg(feature = "aster")]
  pub(crate) aster_secret_key: alloc::string::String,

  #[allow(unused, reason = "implementation is almost finished")]
  #[cfg(feature = "hyperliquid")]
  #[from_vars(map_hyperliquid_sk)]
  pub(crate) hyperliquid_sk: [u8; 32],

  #[cfg(feature = "mercado-pago")]
  pub(crate) mercado_pago_client_id: alloc::string::String,
  #[cfg(feature = "mercado-pago")]
  pub(crate) mercado_pago_client_secret: alloc::string::String,

  #[cfg(feature = "olist")]
  pub(crate) olist_access_token: alloc::string::String,
  #[cfg(feature = "olist")]
  pub(crate) olist_client_id: alloc::string::String,
  #[cfg(feature = "olist")]
  pub(crate) olist_client_secret: alloc::string::String,

  #[allow(unused, reason = "a new testnet accounts needs to be created")]
  #[cfg(feature = "ed25519-dalek")]
  #[from_vars(map_solana_sk)]
  pub(crate) solana_sk: crate::blockchain::solana::SolanaAddressHash,

  #[cfg(feature = "super-frete")]
  pub(crate) super_frete_token: alloc::string::String,

  #[cfg(feature = "vault")]
  pub(crate) vault_token: alloc::string::String,
  #[cfg(feature = "vault")]
  pub(crate) vault_uri: alloc::string::String,
}

#[cfg(feature = "hyperliquid")]
fn map_hyperliquid_sk(var: alloc::string::String) -> wtx::Result<[u8; 32]> {
  let mut rslt = [0; 32];
  let _ = wtx::de::decode_hex(var.as_bytes(), &mut rslt).unwrap();
  Ok(rslt)
}

#[cfg(feature = "ed25519-dalek")]
fn map_solana_sk(
  var: alloc::string::String,
) -> wtx::Result<crate::blockchain::solana::SolanaAddressHash> {
  let mut buffer = crate::blockchain::solana::SolanaAddressHash::default();
  let _ = wtx::de::decode_hex(var.as_bytes(), &mut buffer).unwrap();
  Ok(buffer)
}
