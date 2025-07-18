//! Solana is a public blockchain platform with smart contract functionality.
//!
//! <https://docs.solana.com/developing/clients/jsonrpc-api>
//!
//! ```rust,no_run
//! # async fn fun() -> wtx_apis::Result<()> {
//! use wtx::{client_api_framework::network::HttpParams, de::dnsn::SerdeJson};
//! use wtx_apis::blockchain::solana::{PkgsAux, Solana};
//!
//! let mut pkgs_aux =
//!   PkgsAux::from_minimum(Solana::new(None), SerdeJson, HttpParams::from_uri("URL".into()));
//! let _ = pkgs_aux.get_slot().data(None).build();
//! # Ok(()) }
//! ```

#[macro_use]
mod macros;

mod account;
mod address_lookup_table_account;
mod block;
mod filter;
#[cfg(all(test, feature = "_integration-tests", feature = "std"))]
mod integration_tests;
mod notification;
mod pkg;
pub mod program;
mod reward;
mod short_vec;
mod slot_update;
mod transaction;

pub use account::*;
pub use address_lookup_table_account::*;
pub use block::*;
pub use filter::*;
pub use notification::*;
pub use pkg::*;
pub use reward::*;
pub use slot_update::*;
pub use transaction::*;
use wtx::{
  client_api_framework::{
    Api,
    misc::{Pair, PairMut, RequestThrottling},
    network::{HttpParams, transport::SendingReceivingTransport},
    pkg::Package,
  },
  collection::ArrayStringU8,
  de::protocol::{JsonRpcDecoder, JsonRpcEncoder},
  misc::FnMutFut,
};

pub(crate) type Epoch = u64;
pub(crate) type SolanaProgramName = ArrayStringU8<32>;

_create_blockchain_constants!(
  pub address_hash: SolanaAddressHash = 32,
  pub address_hash_str: SolanaAddressHashStr = 44,
  pub block_hash: SolanaBlockhash = 32,
  pub block_hash_str: SolanaBlockhashStr = 44,
  pub signature_hash: SolanaSignatureHash = 64,
  pub signature_hash_str: SolanaSignatureHashStr = 90,
  pub transaction_hash: SolanaTransactionHash = 64,
  pub transaction_hash_str: SolanaTransactionHashStr = 90
);

#[derive(Clone, Debug)]
#[doc = _generic_api_doc!()]
#[wtx::api(error(crate::Error), pkgs_aux(PkgsAux), transport(http, ws))]
pub struct Solana {
  /// If some, tells that each request must respect calling intervals.
  pub rt: Option<RequestThrottling>,
}

impl Solana {
  /// If desired, it is possible to instantiate directly instead of using this method.
  pub const fn new(rt: Option<RequestThrottling>) -> Self {
    Self { rt }
  }

  #[doc(hidden)]
  pub async fn check_confirmation<'th, A, DRSR, T>(
    pair: &mut PairMut<'_, HttpPkgsAux<A, DRSR>, T>,
    tx_hash: &'th str,
  ) -> crate::Result<bool>
  where
    A: Api<Error = crate::Error>,
    T: SendingReceivingTransport<HttpParams>,
    GetSignatureStatusesPkg<JsonRpcEncoder<GetSignatureStatusesReq<[&'th str; 1]>>>:
      for<'de> Package<
          A,
          DRSR,
          T::Inner,
          HttpParams,
          ExternalResponseContent<'de> = JsonRpcDecoder<GetSignatureStatusesRes>,
        >,
  {
    let signatures = [tx_hash];
    if let Some(Some(GetSignatureStatuses {
      confirmation_status: Commitment::Finalized,
      err,
      ..
    })) = pair
      .trans
      .send_pkg_recv_decode_contained(
        &mut pair.pkgs_aux.get_signature_statuses().data(signatures, None).build(),
        &mut pair.pkgs_aux,
      )
      .await?
      .result?
      .value
      .into_iter()
      .next()
    {
      if let Some(elem) = err { Err(crate::Error::SolanaTxError(elem)) } else { Ok(true) }
    } else {
      Ok(false)
    }
  }

  /// If existing, extracts the parsed spl token account ([program::spl_token::MintAccount]) out of
  /// a generic [AccountData].
  pub fn spl_token_mint_account(
    account_data: &AccountData,
  ) -> crate::Result<&program::spl_token::MintAccount> {
    if let Some(program::spl_token::GenericAccount::Mint(elem)) =
      Self::spl_token_account(account_data)
    {
      Ok(elem)
    } else {
      Err(crate::Error::SolanaAccountIsNotSplTokenMint)
    }
  }

  /// If existing, extracts the parsed spl token account out of a generic [AccountData].
  pub fn spl_token_normal_account(
    account_data: &AccountData,
  ) -> crate::Result<&program::spl_token::TokenAccount> {
    if let Some(program::spl_token::GenericAccount::Account(elem)) =
      Self::spl_token_account(account_data)
    {
      Ok(elem)
    } else {
      Err(crate::Error::SolanaAccountIsNotSplToken)
    }
  }

  /// Sometimes a received blockhash is not valid so this function tries to perform additional calls
  /// with different blockhashes.
  pub async fn try_with_blockhashes<AUX, API, DRSR, E, O, T>(
    mut aux: AUX,
    additional_tries: u8,
    initial_blockhash: SolanaBlockhash,
    pair: &mut Pair<HttpPkgsAux<API, DRSR>, T>,
    mut cb: impl for<'any> FnMutFut<
      (u8, &'any mut AUX, SolanaBlockhash, &'any mut Pair<HttpPkgsAux<API, DRSR>, T>),
      Result = Result<O, E>,
    >,
  ) -> Result<O, E>
  where
    API: Api<Error = crate::Error>,
    E: From<crate::Error>,
    T: SendingReceivingTransport<HttpParams>,
    GetLatestBlockhashPkg<JsonRpcEncoder<GetLatestBlockhashReq>>: for<'de> Package<
        API,
        DRSR,
        T::Inner,
        HttpParams,
        ExternalResponseContent<'de> = JsonRpcDecoder<GetLatestBlockhashRes>,
      >,
  {
    macro_rules! local_blockhash {
      ($local_pair:expr) => {
        $local_pair
          .trans
          .send_pkg_recv_decode_contained(
            &mut $local_pair.pkgs_aux.get_latest_blockhash().data(None).build(),
            &mut $local_pair.pkgs_aux,
          )
          .await
          .map_err(Into::into)?
          .result
          .map_err(Into::into)?
          .value
          .blockhash
      };
    }
    match cb.call((0, &mut aux, initial_blockhash, pair)).await {
      Err(err) => {
        if let Some(n) = additional_tries.checked_sub(1) {
          let mut opt = None;
          for idx in 1..=n {
            let local_blockhash = local_blockhash!(pair);
            if let Ok(elem) = cb.call((idx, &mut aux, local_blockhash, pair)).await {
              opt = Some(elem);
              break;
            }
          }
          if let Some(elem) = opt {
            Ok(elem)
          } else {
            let local_blockhash = local_blockhash!(pair);
            let last = cb.call((additional_tries, &mut aux, local_blockhash, pair)).await?;
            Ok(last)
          }
        } else {
          Err(err)
        }
      }
      Ok(elem) => Ok(elem),
    }
  }

  fn spl_token_account(account_data: &AccountData) -> Option<&program::spl_token::GenericAccount> {
    if let &AccountData::Json(AccountDataJson {
      parsed: AccountDataJsonParsed::SplTokenAccount(ref spl_token_account),
      ..
    }) = account_data
    {
      Some(spl_token_account)
    } else {
      None
    }
  }
}

impl Api for Solana {
  type Error = crate::Error;
  type Id = SolanaId;

  async fn before_sending(&mut self) -> Result<(), Self::Error> {
    if let Some(ref mut rt) = self.rt {
      rt.rc.update_params(&rt.rl).await?;
    }
    Ok(())
  }
}

wtx::create_packages_aux_wrapper!();
