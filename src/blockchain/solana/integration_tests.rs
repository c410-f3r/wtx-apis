use crate::blockchain::solana::{
  AccountEncoding, AccountSubscribeConfig, Commitment, DataSlice, Filter, GetAccountInfoConfig,
  GetLeaderScheduleConfig, GetProgramAccountsConfig, GetTokenAccountsByOwnerConfig,
  GetVoteAccountsConfig, Memcmp, MemcmpEncodedBytes, MintOrProgramId, PkgsAux, Solana,
  SolanaMutPkgsAux,
};
#[cfg(feature = "ed25519-dalek")]
use crate::blockchain::{solana::SolanaAddressHash, ConfirmTransactionOptions};
use core::time::Duration;
use std::sync::LazyLock;
use tokio::sync::Mutex;
use wtx::{
  client_api_framework::{
    dnsn::SerdeJson,
    misc::{RequestLimit, RequestThrottling},
    network::{transport::Transport, HttpParams, WsParams},
  },
  http::ClientTokioRustls,
  misc::Vector,
};

const HTTP_URI: &str = "https://api.testnet.solana.com:443";
const TO_NORMAL_ACCOUNT: &str = "FiuQrMbFUYka1Goec4wdhoiNq3Ms99cxGrW8JWsWfPnJ";
const TO_SOL_TOKEN_ACCOUNT: &str = "CDqKzghiixHryqny9r8RPJzYfg3hiiF7e8JecsF6fuJw";
const TO_SOL_TOKEN_MINT: &str = "So11111111111111111111111111111111111111112";
const TOKEN_PROGRAM: &str = "TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA";
const WS_URI: &str = "wss://api.testnet.solana.com:443";

#[cfg(feature = "ed25519-dalek")]
static ALICE_PK: LazyLock<SolanaAddressHash> = LazyLock::new(|| {
  let mut buffer = SolanaAddressHash::default();
  for (s, b) in std::env::var("ALICE_PK").unwrap().split(',').zip(&mut buffer) {
    *b = s.parse().unwrap();
  }
  buffer
});
#[cfg(feature = "ed25519-dalek")]
static ALICE_SK: LazyLock<SolanaAddressHash> = LazyLock::new(|| {
  let mut buffer = SolanaAddressHash::default();
  for (s, b) in std::env::var("ALICE_SK").unwrap().split(',').zip(&mut buffer) {
    *b = s.parse().unwrap();
  }
  buffer
});
#[cfg(feature = "ed25519-dalek")]
static BOB_PK: SolanaAddressHash = [
  24, 147, 209, 196, 197, 185, 156, 48, 170, 96, 192, 119, 193, 150, 129, 12, 221, 102, 119, 84,
  33, 221, 67, 224, 185, 107, 130, 157, 207, 85, 161, 30,
];
static CLIENT: LazyLock<ClientTokioRustls> =
  LazyLock::new(|| ClientTokioRustls::tokio_rustls(1).build());
static SOLANA: LazyLock<Mutex<Solana>> = LazyLock::new(|| {
  Mutex::new(Solana::new(Some(RequestThrottling::from_rl(
    RequestLimit::new(3, Duration::from_secs(1)).unwrap(),
  ))))
});

create_http_test!(
  &mut *SOLANA.lock().await,
  http(),
  get_account_info,
  &*CLIENT,
  |pkgs_aux, trans| async {
    let _res = trans
      .send_recv_decode_contained(
        &mut pkgs_aux
          .get_account_info()
          .data(
            TO_NORMAL_ACCOUNT,
            Some(GetAccountInfoConfig {
              commitment: None,
              data_slice: None,
              encoding: Some(AccountEncoding::JsonParsed),
            }),
          )
          .build(),
        pkgs_aux,
      )
      .await
      .unwrap()
      .result
      .unwrap()
      .value
      .unwrap();
  }
);

create_http_test!(
  &mut *SOLANA.lock().await,
  http(),
  get_balance,
  &*CLIENT,
  |pkgs_aux, trans| async {
    let _res = trans
      .send_recv_decode_contained(
        &mut pkgs_aux.get_balance().data(TO_NORMAL_ACCOUNT, None).build(),
        pkgs_aux,
      )
      .await
      .unwrap()
      .result
      .unwrap();
  }
);

create_http_test!(
  &mut *SOLANA.lock().await,
  http(),
  get_blocks,
  &*CLIENT,
  |pkgs_aux, trans| async {
    let slot = slot(pkgs_aux, trans).await;
    let _res = trans
      .send_recv_decode_contained(
        &mut pkgs_aux.get_blocks().data(slot, None, None).build(),
        pkgs_aux,
      )
      .await
      .unwrap()
      .result
      .unwrap();
  }
);

create_http_test!(
  &mut *SOLANA.lock().await,
  http(),
  get_blocks_with_limit,
  &*CLIENT,
  |pkgs_aux, trans| async {
    let slot = slot(pkgs_aux, trans).await;
    let _res = trans
      .send_recv_decode_contained(
        &mut pkgs_aux.get_blocks_with_limit().data(slot, 1, None).build(),
        pkgs_aux,
      )
      .await
      .unwrap()
      .result
      .unwrap();
  }
);

create_http_test!(
  &mut *SOLANA.lock().await,
  http(),
  get_block_height,
  &*CLIENT,
  |pkgs_aux, trans| async {
    let _res = trans
      .send_recv_decode_contained(&mut pkgs_aux.get_block_height().data(None).build(), pkgs_aux)
      .await
      .unwrap()
      .result
      .unwrap();
  }
);

create_http_test!(
  &mut *SOLANA.lock().await,
  http(),
  get_block_commitment,
  &*CLIENT,
  |pkgs_aux, trans| async {
    let slot = slot(pkgs_aux, trans).await;
    let _res = trans
      .send_recv_decode_contained(&mut pkgs_aux.get_block_commitment().data(slot).build(), pkgs_aux)
      .await
      .unwrap()
      .result
      .unwrap();
  }
);

create_http_test!(
  &mut *SOLANA.lock().await,
  http(),
  get_block_production,
  &*CLIENT,
  |pkgs_aux, trans| async {
    let _res = trans
      .send_recv_decode_contained(&mut pkgs_aux.get_block_production().data(None).build(), pkgs_aux)
      .await
      .unwrap()
      .result
      .unwrap();
  }
);

create_http_test!(
  &mut *SOLANA.lock().await,
  http(),
  get_block_time,
  &*CLIENT,
  |pkgs_aux, trans| async {
    let slot = slot(pkgs_aux, trans).await;
    let _res = trans
      .send_recv_decode_contained(&mut pkgs_aux.get_block_time().data(slot).build(), pkgs_aux)
      .await
      .unwrap()
      .result
      .unwrap();
  }
);

create_http_test!(
  &mut *SOLANA.lock().await,
  http(),
  get_cluster_nodes,
  &*CLIENT,
  |pkgs_aux, trans| async {
    let _res = trans
      .send_recv_decode_contained(&mut pkgs_aux.get_cluster_nodes().build(), pkgs_aux)
      .await
      .unwrap()
      .result
      .unwrap();
  }
);

create_http_test!(
  &mut *SOLANA.lock().await,
  http(),
  get_epoch_info,
  &*CLIENT,
  |pkgs_aux, trans| async {
    let _res = trans
      .send_recv_decode_contained(&mut pkgs_aux.get_epoch_info().data(None, None).build(), pkgs_aux)
      .await
      .unwrap()
      .result
      .unwrap();
  }
);

create_http_test!(
  &mut *SOLANA.lock().await,
  http(),
  get_epoch_schedule,
  &*CLIENT,
  |pkgs_aux, trans| async {
    let _res = trans
      .send_recv_decode_contained(&mut pkgs_aux.get_epoch_schedule().build(), pkgs_aux)
      .await
      .unwrap()
      .result
      .unwrap();
  }
);

#[cfg(feature = "ed25519-dalek")]
create_http_test!(
  &mut *SOLANA.lock().await,
  http(),
  get_fee_for_message,
  &*CLIENT,
  |pkgs_aux, trans| async {
    let blockhash = latest_blockhash(pkgs_aux, trans).await;
    assert_eq!(
      trans
        .send_recv_decode_contained(
          &mut pkgs_aux
            .get_fee_for_message()
            .data(None, &transfer_message(blockhash, *ALICE_PK))
            .unwrap()
            .build(),
          pkgs_aux
        )
        .await
        .unwrap()
        .result
        .unwrap()
        .value
        .unwrap(),
      5000
    );
  }
);

create_http_test!(
  &mut *SOLANA.lock().await,
  http(),
  get_first_available_block,
  &*CLIENT,
  |pkgs_aux, trans| async {
    let _res = trans
      .send_recv_decode_contained(&mut pkgs_aux.get_first_available_block().build(), pkgs_aux)
      .await
      .unwrap()
      .result
      .unwrap();
  }
);

create_http_test!(
  &mut *SOLANA.lock().await,
  http(),
  get_genesis_hash,
  &*CLIENT,
  |pkgs_aux, trans| async {
    let _res = trans
      .send_recv_decode_contained(&mut pkgs_aux.get_genesis_hash().build(), pkgs_aux)
      .await
      .unwrap()
      .result
      .unwrap();
  }
);

create_http_test!(
  &mut *SOLANA.lock().await,
  http(),
  get_health,
  &*CLIENT,
  |pkgs_aux, trans| async {
    let _res = trans
      .send_recv_decode_contained(&mut pkgs_aux.get_health().build(), pkgs_aux)
      .await
      .unwrap()
      .result
      .unwrap();
  }
);

create_http_test!(
  &mut *SOLANA.lock().await,
  http(),
  get_highest_snapshot_slot,
  &*CLIENT,
  |pkgs_aux, trans| async {
    let _res = trans
      .send_recv_decode_contained(&mut pkgs_aux.get_highest_snapshot_slot().build(), pkgs_aux)
      .await
      .unwrap()
      .result
      .unwrap();
  }
);

create_http_test!(
  &mut *SOLANA.lock().await,
  http(),
  get_identity,
  &*CLIENT,
  |pkgs_aux, trans| async {
    let _res = trans
      .send_recv_decode_contained(&mut pkgs_aux.get_identity().build(), pkgs_aux)
      .await
      .unwrap()
      .result
      .unwrap();
  }
);

create_http_test!(
  &mut *SOLANA.lock().await,
  http(),
  get_inflation_governor,
  &*CLIENT,
  |pkgs_aux, trans| async {
    let _res = trans
      .send_recv_decode_contained(
        &mut pkgs_aux.get_inflation_governor().data(None).build(),
        pkgs_aux,
      )
      .await
      .unwrap()
      .result
      .unwrap();
  }
);

create_http_test!(
  &mut *SOLANA.lock().await,
  http(),
  get_inflation_rate,
  &*CLIENT,
  |pkgs_aux, trans| async {
    let _res = trans
      .send_recv_decode_contained(&mut pkgs_aux.get_inflation_rate().build(), pkgs_aux)
      .await
      .unwrap()
      .result
      .unwrap();
  }
);

create_http_test!(
  &mut *SOLANA.lock().await,
  http(),
  get_leader_schedule,
  &*CLIENT,
  |pkgs_aux, trans| async {
    let config: Option<GetLeaderScheduleConfig<&str>> = None;
    let _res = trans
      .send_recv_decode_contained(
        &mut pkgs_aux.get_leader_schedule().data(None, config).build(),
        pkgs_aux,
      )
      .await
      .unwrap()
      .result
      .unwrap();
  }
);

create_http_test!(
  &mut *SOLANA.lock().await,
  http(),
  get_max_retransmit_slot,
  &*CLIENT,
  |pkgs_aux, trans| async {
    let _res = trans
      .send_recv_decode_contained(&mut pkgs_aux.get_max_retransmit_slot().build(), pkgs_aux)
      .await
      .unwrap()
      .result
      .unwrap();
  }
);

create_http_test!(
  &mut *SOLANA.lock().await,
  http(),
  get_max_shred_insert_slot,
  &*CLIENT,
  |pkgs_aux, trans| async {
    let _res = trans
      .send_recv_decode_contained(&mut pkgs_aux.get_max_shred_insert_slot().build(), pkgs_aux)
      .await
      .unwrap()
      .result
      .unwrap();
  }
);

create_http_test!(
  &mut *SOLANA.lock().await,
  http(),
  get_minimum_balance_for_rent_exemption,
  &*CLIENT,
  |pkgs_aux, trans| async {
    let _res = trans
      .send_recv_decode_contained(
        &mut pkgs_aux.get_minimum_balance_for_rent_exemption().data(100, None).build(),
        pkgs_aux,
      )
      .await
      .unwrap()
      .result
      .unwrap();
  }
);

create_http_test!(
  &mut *SOLANA.lock().await,
  http(),
  get_multiple_accounts,
  &*CLIENT,
  |pkgs_aux, trans| async {
    let _res = trans
      .send_recv_decode_contained(
        &mut pkgs_aux
          .get_multiple_accounts()
          .data([TO_NORMAL_ACCOUNT, TO_SOL_TOKEN_ACCOUNT], None)
          .build(),
        pkgs_aux,
      )
      .await
      .unwrap()
      .result
      .unwrap();
  }
);

create_http_test!(
  &mut *SOLANA.lock().await,
  http(),
  get_program_accounts,
  &*CLIENT,
  |pkgs_aux, trans| async {
    assert_eq!(
      trans
        .send_recv_decode_contained(
          &mut pkgs_aux
            .get_program_accounts()
            .data(
              TOKEN_PROGRAM,
              Some(GetProgramAccountsConfig {
                commitment: None,
                data_slice: Some(DataSlice { length: 32, offset: 0 }),
                encoding: Some(AccountEncoding::Base64),
                filters: Some(&[
                  Filter::DataSize(165),
                  Filter::Memcmp(Memcmp {
                    bytes: MemcmpEncodedBytes::Base58(TO_NORMAL_ACCOUNT),
                    offset: 32,
                  }),
                ]),
                min_context_slot: Some(2),
              }),
            )
            .build(),
          pkgs_aux
        )
        .await
        .unwrap()
        .result
        .unwrap()
        .len(),
      1
    );
  }
);

create_http_test!(
  &mut *SOLANA.lock().await,
  http(),
  get_recent_performance_samples,
  &*CLIENT,
  |pkgs_aux, trans| async {
    let _res = trans
      .send_recv_decode_contained(
        &mut pkgs_aux.get_recent_performance_samples().data(None).build(),
        pkgs_aux,
      )
      .await
      .unwrap()
      .result
      .unwrap();
  }
);

create_http_test!(
  &mut *SOLANA.lock().await,
  http(),
  get_signatures_for_addresses,
  &*CLIENT,
  |pkgs_aux, trans| async {
    let _res = trans
      .send_recv_decode_contained(
        &mut pkgs_aux.get_signatures_for_address().data(TO_NORMAL_ACCOUNT, None).build(),
        pkgs_aux,
      )
      .await
      .unwrap()
      .result
      .unwrap();
  }
);

create_http_test!(
  &mut *SOLANA.lock().await,
  http(),
  get_slot_leader,
  &*CLIENT,
  |pkgs_aux, trans| async {
    let _res = trans
      .send_recv_decode_contained(&mut pkgs_aux.get_slot_leader().data(None).build(), pkgs_aux)
      .await
      .unwrap()
      .result
      .unwrap();
  }
);

create_http_test!(
  &mut *SOLANA.lock().await,
  http(),
  get_slot_leaders,
  &*CLIENT,
  |pkgs_aux, trans| async {
    let slot = slot(pkgs_aux, trans).await;
    let _res = trans
      .send_recv_decode_contained(&mut pkgs_aux.get_slot_leaders().data(slot, 2).build(), pkgs_aux)
      .await
      .unwrap()
      .result
      .unwrap();
  }
);

create_http_test!(&mut *SOLANA.lock().await, http(), get_slot, &*CLIENT, |pkgs_aux, trans| async {
  let _res = trans
    .send_recv_decode_contained(&mut pkgs_aux.get_slot().data(None).build(), pkgs_aux)
    .await
    .unwrap();
});

create_http_test!(
  &mut *SOLANA.lock().await,
  http(),
  get_supply,
  &*CLIENT,
  |pkgs_aux, trans| async {
    let _res = trans
      .send_recv_decode_contained(&mut pkgs_aux.get_supply().data(None).build(), pkgs_aux)
      .await
      .unwrap()
      .result
      .unwrap();
  }
);

create_http_test!(
  &mut *SOLANA.lock().await,
  http(),
  get_token_account_balance,
  &*CLIENT,
  |pkgs_aux, trans| async {
    let _res = trans
      .send_recv_decode_contained(
        &mut pkgs_aux.get_token_account_balance().data(TO_SOL_TOKEN_ACCOUNT, None).build(),
        pkgs_aux,
      )
      .await
      .unwrap()
      .result
      .unwrap();
  }
);

create_http_test!(
  &mut *SOLANA.lock().await,
  http(),
  get_token_accounts_by_delegate,
  &*CLIENT,
  |pkgs_aux, trans| async {
    let _res = trans
      .send_recv_decode_contained(
        &mut pkgs_aux
          .get_token_accounts_by_delegate()
          .data(TO_NORMAL_ACCOUNT, MintOrProgramId::Mint(TO_SOL_TOKEN_MINT), None)
          .build(),
        pkgs_aux,
      )
      .await
      .unwrap()
      .result
      .unwrap();
  }
);

create_http_test!(
  &mut *SOLANA.lock().await,
  http(),
  get_token_accounts_by_owner,
  &*CLIENT,
  |pkgs_aux, trans| async {
    let _res = trans
      .send_recv_decode_contained(
        &mut pkgs_aux
          .get_token_accounts_by_owner()
          .data(
            TO_NORMAL_ACCOUNT,
            MintOrProgramId::Mint(TO_SOL_TOKEN_MINT),
            Some(GetTokenAccountsByOwnerConfig {
              commitment: None,
              data_slice: None,
              encoding: Some(AccountEncoding::JsonParsed),
              min_context_slot: None,
            }),
          )
          .build(),
        pkgs_aux,
      )
      .await
      .unwrap()
      .result
      .unwrap()
      .value[0]
      .pubkey;
  }
);

create_http_test!(
  &mut *SOLANA.lock().await,
  http(),
  get_token_largest_accounts,
  &*CLIENT,
  |pkgs_aux, trans| async {
    let _res = trans
      .send_recv_decode_contained(
        &mut pkgs_aux.get_token_largest_accounts().data(TO_SOL_TOKEN_MINT, None).build(),
        pkgs_aux,
      )
      .await
      .unwrap()
      .result
      .unwrap();
  }
);

create_http_test!(
  &mut *SOLANA.lock().await,
  http(),
  get_token_supply,
  &*CLIENT,
  |pkgs_aux, trans| async {
    let _res = trans
      .send_recv_decode_contained(
        &mut pkgs_aux.get_token_supply().data(TO_SOL_TOKEN_MINT, None).build(),
        pkgs_aux,
      )
      .await
      .unwrap()
      .result
      .unwrap();
  }
);

create_http_test!(
  &mut *SOLANA.lock().await,
  http(),
  get_transaction_count,
  &*CLIENT,
  |pkgs_aux, trans| async {
    let _res = trans
      .send_recv_decode_contained(
        &mut pkgs_aux.get_transaction_count().data(None).build(),
        pkgs_aux,
      )
      .await
      .unwrap()
      .result
      .unwrap();
  }
);

create_http_test!(
  &mut *SOLANA.lock().await,
  http(),
  get_version,
  &*CLIENT,
  |pkgs_aux, trans| async {
    let _res = trans
      .send_recv_decode_contained(&mut pkgs_aux.get_version().build(), pkgs_aux)
      .await
      .unwrap()
      .result
      .unwrap()
      .feature_set;
  }
);

create_http_test!(
  &mut *SOLANA.lock().await,
  http(),
  get_vote_accounts,
  &*CLIENT,
  |pkgs_aux, trans| async {
    let config: Option<GetVoteAccountsConfig<&str>> = None;
    let _res = trans
      .send_recv_decode_contained(&mut pkgs_aux.get_vote_accounts().data(config).build(), pkgs_aux)
      .await
      .unwrap()
      .result
      .unwrap();
  }
);

create_http_test!(
  &mut *SOLANA.lock().await,
  http(),
  is_blockhash_valid,
  &*CLIENT,
  |pkgs_aux, trans| async {
    let _res = trans
      .send_recv_decode_contained(
        &mut pkgs_aux.is_blockhash_valid().data(TO_NORMAL_ACCOUNT, None).build(),
        pkgs_aux,
      )
      .await
      .unwrap()
      .result
      .unwrap();
  }
);

create_http_test!(
  &mut *SOLANA.lock().await,
  http(),
  minimum_ledger_slot,
  &*CLIENT,
  |pkgs_aux, trans| async {
    let _res = trans
      .send_recv_decode_contained(&mut pkgs_aux.minimum_ledger_slot().build(), pkgs_aux)
      .await
      .unwrap()
      .result
      .unwrap();
  }
);

#[cfg(feature = "ed25519-dalek")]
create_http_test!(
  &mut *SOLANA.lock().await,
  http(),
  http_get_latest_blockhash_send_transaction_and_get_transaction,
  &*CLIENT,
  |pkgs_aux, trans| async {
    let from_keypair = ed25519_dalek::SigningKey::from_bytes(&ALICE_SK);
    let blockhash = latest_blockhash(pkgs_aux, trans).await;
    let tx = crate::blockchain::solana::TransactionInput::new(
      &mut pkgs_aux.byte_buffer,
      blockhash,
      transfer_message(blockhash, *ALICE_PK).into(),
      &[from_keypair],
    )
    .unwrap();
    let tx_hash = trans
      .send_recv_decode_contained(
        &mut pkgs_aux.send_transaction().data(None, &tx).unwrap().build(),
        pkgs_aux,
      )
      .await
      .unwrap()
      .result
      .unwrap();
    Solana::confirm_transaction(
      ConfirmTransactionOptions::default(),
      &mut (&mut *pkgs_aux, &mut *trans).into(),
      &tx_hash,
    )
    .await
    .unwrap();

    let _res = trans
      .send_recv_decode_contained(
        &mut pkgs_aux
          .get_transaction()
          .data(
            tx_hash.as_str(),
            Some(crate::blockchain::solana::GetTransactionConfig {
              commitment: Some(Commitment::Finalized),
              encoding: Some(crate::blockchain::solana::TransactionEncoding::Base64),
              max_supported_transaction_version: Some(0),
            }),
          )
          .build(),
        pkgs_aux,
      )
      .await
      .unwrap()
      .result
      .unwrap()
      .transaction;
  }
);

create_http_test!(
  &mut *SOLANA.lock().await,
  http(),
  http_reqs_with_array,
  &*CLIENT,
  |pkgs_aux, trans| async {
    let mut buffer = Vector::new();
    trans
      .send_recv_decode_batch(
        &mut [
          &mut pkgs_aux.get_balance().data(TO_NORMAL_ACCOUNT, None).build(),
          &mut pkgs_aux.get_balance().data(TO_NORMAL_ACCOUNT, None).build(),
        ][..],
        pkgs_aux,
        &mut buffer,
      )
      .await
      .unwrap();
  }
);

create_ws_test!(
  WS_URI,
  &mut *SOLANA.lock().await,
  ws(),
  account_subscribe,
  (account_unsubscribe),
  |pkgs_aux, trans| async {
    [trans
      .send_recv_decode_contained(
        &mut pkgs_aux
          .account_subscribe()
          .data(
            TO_NORMAL_ACCOUNT,
            Some(AccountSubscribeConfig { commitment: None, encoding: None }),
          )
          .build(),
        pkgs_aux,
      )
      .await
      .unwrap()
      .result
      .unwrap()]
  }
);

create_ws_test!(
  WS_URI,
  &mut *SOLANA.lock().await,
  ws(),
  root_subscribe,
  (root_unsubscribe),
  |pkgs_aux, trans| async {
    [trans
      .send_recv_decode_contained(&mut pkgs_aux.root_subscribe().build(), pkgs_aux)
      .await
      .unwrap()
      .result
      .unwrap()]
  }
);

create_ws_test!(
  WS_URI,
  &mut *SOLANA.lock().await,
  ws(),
  slot_subscribe,
  (slot_unsubscribe),
  |pkgs_aux, trans| async {
    [trans
      .send_recv_decode_contained(&mut pkgs_aux.slot_subscribe().build(), pkgs_aux)
      .await
      .unwrap()
      .result
      .unwrap()]
  }
);

create_ws_test!(
  WS_URI,
  &mut *SOLANA.lock().await,
  ws(),
  slot_updates_subscribe,
  (slots_updates_unsubscribe),
  |pkgs_aux, trans| async {
    [trans
      .send_recv_decode_contained(&mut pkgs_aux.slots_updates_subscribe().build(), pkgs_aux)
      .await
      .unwrap()
      .result
      .unwrap()]
  }
);

create_ws_test!(
  WS_URI,
  &mut *SOLANA.lock().await,
  ws(),
  ws_reqs_with_array,
  (account_unsubscribe, account_unsubscribe),
  |pkgs_aux, trans| async {
    let mut buffer = Vector::new();
    trans
      .send_recv_decode_batch(
        &mut [
          &mut pkgs_aux
            .account_subscribe()
            .data(
              TO_NORMAL_ACCOUNT,
              Some(AccountSubscribeConfig {
                commitment: Some(Commitment::Confirmed),
                encoding: Some(AccountEncoding::JsonParsed),
              }),
            )
            .build(),
          &mut pkgs_aux
            .account_subscribe()
            .data(
              TO_NORMAL_ACCOUNT,
              Some(AccountSubscribeConfig {
                commitment: Some(Commitment::Confirmed),
                encoding: Some(AccountEncoding::JsonParsed),
              }),
            )
            .build(),
        ],
        pkgs_aux,
        &mut buffer,
      )
      .await
      .unwrap();
    [*buffer[0].result.as_ref().unwrap(), *buffer[1].result.as_ref().unwrap()]
  }
);

fn http() -> (SerdeJson, HttpParams) {
  (SerdeJson, HttpParams::from_uri(HTTP_URI))
}

#[cfg(feature = "ed25519-dalek")]
async fn latest_blockhash(
  pkgs_aux: &mut SolanaMutPkgsAux<'_, SerdeJson, HttpParams>,
  mut trans: &ClientTokioRustls,
) -> SolanaAddressHash {
  trans
    .send_recv_decode_contained(&mut pkgs_aux.get_latest_blockhash().data(None).build(), pkgs_aux)
    .await
    .unwrap()
    .result
    .unwrap()
    .value
    .blockhash
}

async fn slot(
  pkgs_aux: &mut SolanaMutPkgsAux<'_, SerdeJson, HttpParams>,
  mut trans: &ClientTokioRustls,
) -> u64 {
  trans
    .send_recv_decode_contained(&mut pkgs_aux.get_slot().data(None).build(), pkgs_aux)
    .await
    .unwrap()
    .result
    .unwrap()
}

#[cfg(feature = "ed25519-dalek")]
fn transfer_message(
  blockhash: [u8; 32],
  from_public_key: [u8; 32],
) -> crate::blockchain::solana::MessageInput {
  let transfer = crate::blockchain::solana::InstructionInput {
    accounts: alloc::vec![
      crate::blockchain::solana::InstructionAccountInput {
        pubkey: *ALICE_PK,
        is_signer: true,
        is_writable: true,
      },
      crate::blockchain::solana::InstructionAccountInput {
        pubkey: BOB_PK,
        is_signer: false,
        is_writable: true,
      },
    ],
    data: alloc::vec![2, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
    program_id: [0; 32],
  };
  crate::blockchain::solana::MessageInput::with_params(
    &[],
    &mut <_>::default(),
    &[transfer],
    Some(from_public_key),
    blockhash,
  )
  .unwrap()
}

fn ws() -> (SerdeJson, WsParams) {
  (SerdeJson, WsParams::default())
}
