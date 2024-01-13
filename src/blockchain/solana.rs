//! Solana is a public blockchain platform with smart contract functionality.
//!
//! <https://docs.solana.com/developing/clients/jsonrpc-api>
//!
//! ```rust,no_run
//! # async fn fun() -> wtx_apis::Result<()> {
//! use wtx::client_api_framework::{dnsn::SerdeJson, network::HttpParams};
//! use wtx_apis::blockchain::solana::{PkgsAux, Solana};
//!
//! let mut pkgs_aux =
//!   PkgsAux::from_minimum(Solana::new(None), SerdeJson, HttpParams::from_uri("URL"));
//! let _ = pkgs_aux.get_slot().data(None).build();
//! # Ok(()) }
//! ```

wtx::create_packages_aux_wrapper!();

#[macro_use]
mod macros;

mod account;
mod address_lookup_table_account;
mod block;
mod filter;
#[cfg(all(test, feature = "_integration-tests"))]
mod integration_tests;
mod notification;
mod pkg;
pub mod program;
mod reward;
mod short_vec;
mod slot_update;
mod transaction;

use crate::blockchain::ConfirmTransactionOptions;
pub use account::*;
pub use address_lookup_table_account::*;
use arrayvec::ArrayString;
pub use block::*;
pub use filter::*;
pub use notification::*;
pub use pkg::*;
pub use reward::*;
pub use slot_update::*;
pub use transaction::*;
use wtx::{
  client_api_framework::{
    data_format::{JsonRpcRequest, JsonRpcResponse},
    misc::{Pair, PairMut, RequestThrottling},
    network::{transport::Transport, HttpParams},
    pkg::Package,
    Api,
  },
  misc::{AsyncBounds, FnMutFut},
};

pub(crate) type Epoch = u64;
pub(crate) type SolanaProgramName = ArrayString<32>;

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

#[derive(Debug)]
#[doc = _generic_api_doc!()]
#[wtx_macros::api_types(pkgs_aux(PkgsAux), transport(http, ws))]
pub struct Solana {
  /// If some, tells that each request must respect calling intervals.
  pub rt: Option<RequestThrottling>,
}

impl Solana {
  /// If desired, it is possible to instantiate directly instead of using this method.
  pub fn new(rt: Option<RequestThrottling>) -> Self {
    Self { rt }
  }

  /// Make successive HTTP requests over a period defined in `cto` until the transaction is
  /// successful or expired.
  pub async fn confirm_transaction<'th, A, DRSR, T>(
    cto: ConfirmTransactionOptions,
    pair: &mut PairMut<'_, HttpPkgsAux<A, DRSR>, T>,
    tx_hash: &'th str,
  ) -> Result<(), A::Error>
  where
    A: Api<Error = crate::Error>,
    DRSR: AsyncBounds,
    T: AsyncBounds + Transport<DRSR, Params = HttpParams>,
    GetSignatureStatusesPkg<JsonRpcRequest<GetSignatureStatusesReq<[&'th str; 1]>>>: Package<
      A,
      DRSR,
      T::Params,
      ExternalResponseContent = JsonRpcResponse<GetSignatureStatusesRes>,
    >,
  {
    macro_rules! call {
      () => {{
        let signatures = [tx_hash];
        if let Some(Some(GetSignatureStatuses {
          confirmation_status: Commitment::Finalized, ..
        })) = pair
          .trans
          .send_retrieve_and_decode_contained(
            &mut pair.pkgs_aux.get_signature_statuses().data(signatures, None).build(),
            &mut pair.pkgs_aux,
          )
          .await?
          .result?
          .value
          .get(0)
        {
          true
        } else {
          false
        }
      }};
    }

    match cto {
      ConfirmTransactionOptions::Tries { number } => {
        for _ in 0u16..number {
          if call!() {
            return Ok(());
          }
        }
      }
      ConfirmTransactionOptions::TriesWithInterval { interval, number } => {
        for _ in 0u16..number {
          if call!() {
            return Ok(());
          }
          wtx::misc::sleep(interval).await?;
        }
      }
    }

    Err(crate::Error::CouldNotConfirmTransaction)
  }

  /// If existing, extracts the parsed spl token account ([program::spl_token::MintAccount]) out of
  /// a generic [AccountData].
  pub fn spl_token_mint_account(
    account_data: &AccountData,
  ) -> crate::Result<&program::spl_token::MintAccount> {
    if let Some(program::spl_token::GenericAccount::Mint(ref elem)) =
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
    if let Some(program::spl_token::GenericAccount::Account(ref elem)) =
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
      Result<O, E>,
    >,
  ) -> Result<O, E>
  where
    API: Api<Error = crate::Error>,
    DRSR: AsyncBounds,
    E: From<crate::Error>,
    T: AsyncBounds + Transport<DRSR, Params = HttpParams>,
    GetLatestBlockhashPkg<JsonRpcRequest<GetLatestBlockhashReq>>: Package<
      API,
      DRSR,
      HttpParams,
      ExternalResponseContent = JsonRpcResponse<GetLatestBlockhashRes>,
    >,
  {
    macro_rules! local_blockhash {
      ($local_pair:expr) => {
        $local_pair
          .trans
          .send_retrieve_and_decode_contained(
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
    match cb((0, &mut aux, initial_blockhash, pair)).await {
      Err(err) => {
        if let Some(n) = additional_tries.checked_sub(1) {
          let mut opt = None;
          for idx in 1..=n {
            let local_blockhash = local_blockhash!(pair);
            if let Ok(elem) = cb((idx, &mut aux, local_blockhash, pair)).await {
              opt = Some(elem);
              break;
            }
          }
          if let Some(elem) = opt {
            Ok(elem)
          } else {
            let local_blockhash = local_blockhash!(pair);
            let last = cb((additional_tries, &mut aux, local_blockhash, pair)).await?;
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

  async fn before_sending(&mut self) -> Result<(), Self::Error> {
    if let Some(ref mut rt) = self.rt {
      rt.rc.update_params(&rt.rl).await?;
    }
    Ok(())
  }
}
