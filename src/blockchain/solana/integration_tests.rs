use crate::blockchain::solana::{
  AccountEncoding, AccountSubscribeConfig, Commitment, DataSlice, Filter, GetAccountInfoConfig,
  GetLeaderScheduleConfig, GetProgramAccountsConfig, GetTokenAccountsByOwnerConfig,
  GetVoteAccountsConfig, Memcmp, MemcmpEncodedBytes, MintOrProgramId, PkgsAux, Solana,
  SolanaAddressHash, SolanaMutPkgsAux, TransactionEncoding,
};
use alloc::vec::Vec;
use reqwest::Client;
use std::sync::OnceLock;
use tokio::sync::{MappedMutexGuard, Mutex};
use wtx::{
  client_api_framework::{
    dnsn::SerdeJson,
    network::{transport::Transport, HttpParams, WsParams},
  },
  pool_manager::{Pool as _, SimpleRM, StaticPool},
};

const TO_NORMAL_ACCOUNT: &str = "FiuQrMbFUYka1Goec4wdhoiNq3Ms99cxGrW8JWsWfPnJ";
const TO_SOL_TOKEN_ACCOUNT: &str = "CDqKzghiixHryqny9r8RPJzYfg3hiiF7e8JecsF6fuJw";
const TO_SOL_TOKEN_MINT: &str = "So11111111111111111111111111111111111111112";
const TOKEN_PROGRAM: &str = "TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA";
const WS_URL: &str = "wss://api.testnet.solana.com:443";

create_http_test!(&mut *pool_resource().await, http(), get_account_info, |pkgs_aux, trans| async {
  let _res = trans
    .send_retrieve_and_decode_contained(
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
});

create_http_test!(&mut *pool_resource().await, http(), get_balance, |pkgs_aux, trans| async {
  let _res = trans
    .send_retrieve_and_decode_contained(
      &mut pkgs_aux.get_balance().data(TO_NORMAL_ACCOUNT, None).build(),
      pkgs_aux,
    )
    .await
    .unwrap()
    .result
    .unwrap();
});

create_http_test!(&mut *pool_resource().await, http(), get_blocks, |pkgs_aux, trans| async {
  let slot = slot(pkgs_aux, trans).await;
  let _res = trans
    .send_retrieve_and_decode_contained(
      &mut pkgs_aux.get_blocks().data(slot, None, None).build(),
      pkgs_aux,
    )
    .await
    .unwrap()
    .result
    .unwrap();
});

create_http_test!(
  &mut *pool_resource().await,
  http(),
  get_blocks_with_limit,
  |pkgs_aux, trans| async {
    let slot = slot(pkgs_aux, trans).await;
    let _res = trans
      .send_retrieve_and_decode_contained(
        &mut pkgs_aux.get_blocks_with_limit().data(slot, 1, None).build(),
        pkgs_aux,
      )
      .await
      .unwrap()
      .result
      .unwrap();
  }
);

create_http_test!(&mut *pool_resource().await, http(), get_block_height, |pkgs_aux, trans| async {
  let _res = trans
    .send_retrieve_and_decode_contained(
      &mut pkgs_aux.get_block_height().data(None).build(),
      pkgs_aux,
    )
    .await
    .unwrap()
    .result
    .unwrap();
});

create_http_test!(
  &mut *pool_resource().await,
  http(),
  get_block_commitment,
  |pkgs_aux, trans| async {
    let slot = slot(pkgs_aux, trans).await;
    let _res = trans
      .send_retrieve_and_decode_contained(
        &mut pkgs_aux.get_block_commitment().data(slot).build(),
        pkgs_aux,
      )
      .await
      .unwrap()
      .result
      .unwrap();
  }
);

create_http_test!(
  &mut *pool_resource().await,
  http(),
  get_block_production,
  |pkgs_aux, trans| async {
    let _res = trans
      .send_retrieve_and_decode_contained(
        &mut pkgs_aux.get_block_production().data(None).build(),
        pkgs_aux,
      )
      .await
      .unwrap()
      .result
      .unwrap();
  }
);

create_http_test!(&mut *pool_resource().await, http(), get_block_time, |pkgs_aux, trans| async {
  let slot = slot(pkgs_aux, trans).await;
  let _res = trans
    .send_retrieve_and_decode_contained(&mut pkgs_aux.get_block_time().data(slot).build(), pkgs_aux)
    .await
    .unwrap()
    .result
    .unwrap();
});

create_http_test!(
  &mut *pool_resource().await,
  http(),
  get_cluster_nodes,
  |pkgs_aux, trans| async {
    let _res = trans
      .send_retrieve_and_decode_contained(&mut pkgs_aux.get_cluster_nodes().build(), pkgs_aux)
      .await
      .unwrap()
      .result
      .unwrap();
  }
);

create_http_test!(&mut *pool_resource().await, http(), get_epoch_info, |pkgs_aux, trans| async {
  let _res = trans
    .send_retrieve_and_decode_contained(
      &mut pkgs_aux.get_epoch_info().data(None, None).build(),
      pkgs_aux,
    )
    .await
    .unwrap()
    .result
    .unwrap();
});

create_http_test!(
  &mut *pool_resource().await,
  http(),
  get_epoch_schedule,
  |pkgs_aux, trans| async {
    let _res = trans
      .send_retrieve_and_decode_contained(&mut pkgs_aux.get_epoch_schedule().build(), pkgs_aux)
      .await
      .unwrap()
      .result
      .unwrap();
  }
);

#[cfg(all(feature = "ed25519-dalek", feature = "solana-program"))]
create_http_test!(
  &mut *pool_resource().await,
  http(),
  get_fee_for_message,
  |pkgs_aux, trans| async {
    let from_keypair = ed25519_dalek::Keypair::from_bytes(&alice_keypair()[..]).unwrap();
    let from_public_key = from_keypair.public.to_bytes();
    let blockhash = latest_blockhash(pkgs_aux, trans).await;
    assert_eq!(
      trans
        .send_retrieve_and_decode_contained(
          &mut pkgs_aux
            .get_fee_for_message()
            .data(None, &transfer_message(blockhash, from_public_key))
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
  &mut *pool_resource().await,
  http(),
  get_first_available_block,
  |pkgs_aux, trans| async {
    let _res = trans
      .send_retrieve_and_decode_contained(
        &mut pkgs_aux.get_first_available_block().build(),
        pkgs_aux,
      )
      .await
      .unwrap()
      .result
      .unwrap();
  }
);

create_http_test!(&mut *pool_resource().await, http(), get_genesis_hash, |pkgs_aux, trans| async {
  let _res = trans
    .send_retrieve_and_decode_contained(&mut pkgs_aux.get_genesis_hash().build(), pkgs_aux)
    .await
    .unwrap()
    .result
    .unwrap();
});

create_http_test!(&mut *pool_resource().await, http(), get_health, |pkgs_aux, trans| async {
  let _res = trans
    .send_retrieve_and_decode_contained(&mut pkgs_aux.get_health().build(), pkgs_aux)
    .await
    .unwrap()
    .result
    .unwrap();
});

create_http_test!(
  &mut *pool_resource().await,
  http(),
  get_highest_snapshot_slot,
  |pkgs_aux, trans| async {
    let _res = trans
      .send_retrieve_and_decode_contained(
        &mut pkgs_aux.get_highest_snapshot_slot().build(),
        pkgs_aux,
      )
      .await
      .unwrap()
      .result
      .unwrap();
  }
);

create_http_test!(&mut *pool_resource().await, http(), get_identity, |pkgs_aux, trans| async {
  let _res = trans
    .send_retrieve_and_decode_contained(&mut pkgs_aux.get_identity().build(), pkgs_aux)
    .await
    .unwrap()
    .result
    .unwrap();
});

create_http_test!(
  &mut *pool_resource().await,
  http(),
  get_inflation_governor,
  |pkgs_aux, trans| async {
    let _res = trans
      .send_retrieve_and_decode_contained(
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
  &mut *pool_resource().await,
  http(),
  get_inflation_rate,
  |pkgs_aux, trans| async {
    let _res = trans
      .send_retrieve_and_decode_contained(&mut pkgs_aux.get_inflation_rate().build(), pkgs_aux)
      .await
      .unwrap()
      .result
      .unwrap();
  }
);

create_http_test!(
  &mut *pool_resource().await,
  http(),
  get_leader_schedule,
  |pkgs_aux, trans| async {
    let config: Option<GetLeaderScheduleConfig<&str>> = None;
    let _res = trans
      .send_retrieve_and_decode_contained(
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
  &mut *pool_resource().await,
  http(),
  get_max_retransmit_slot,
  |pkgs_aux, trans| async {
    let _res = trans
      .send_retrieve_and_decode_contained(&mut pkgs_aux.get_max_retransmit_slot().build(), pkgs_aux)
      .await
      .unwrap()
      .result
      .unwrap();
  }
);

create_http_test!(
  &mut *pool_resource().await,
  http(),
  get_max_shred_insert_slot,
  |pkgs_aux, trans| async {
    let _res = trans
      .send_retrieve_and_decode_contained(
        &mut pkgs_aux.get_max_shred_insert_slot().build(),
        pkgs_aux,
      )
      .await
      .unwrap()
      .result
      .unwrap();
  }
);

create_http_test!(
  &mut *pool_resource().await,
  http(),
  get_minimum_balance_for_rent_exemption,
  |pkgs_aux, trans| async {
    let _res = trans
      .send_retrieve_and_decode_contained(
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
  &mut *pool_resource().await,
  http(),
  get_multiple_accounts,
  |pkgs_aux, trans| async {
    let _res = trans
      .send_retrieve_and_decode_contained(
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
  &mut *pool_resource().await,
  http(),
  get_program_accounts,
  |pkgs_aux, trans| async {
    assert_eq!(
      trans
        .send_retrieve_and_decode_contained(
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
  &mut *pool_resource().await,
  http(),
  get_recent_performance_samples,
  |pkgs_aux, trans| async {
    let _res = trans
      .send_retrieve_and_decode_contained(
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
  &mut *pool_resource().await,
  http(),
  get_signatures_for_addresses,
  |pkgs_aux, trans| async {
    let _res = trans
      .send_retrieve_and_decode_contained(
        &mut pkgs_aux.get_signatures_for_address().data(TO_NORMAL_ACCOUNT, None).build(),
        pkgs_aux,
      )
      .await
      .unwrap()
      .result
      .unwrap();
  }
);

create_http_test!(&mut *pool_resource().await, http(), get_slot_leader, |pkgs_aux, trans| async {
  let _res = trans
    .send_retrieve_and_decode_contained(
      &mut pkgs_aux.get_slot_leader().data(None).build(),
      pkgs_aux,
    )
    .await
    .unwrap()
    .result
    .unwrap();
});

create_http_test!(&mut *pool_resource().await, http(), get_slot_leaders, |pkgs_aux, trans| async {
  let slot = slot(pkgs_aux, trans).await;
  let _res = trans
    .send_retrieve_and_decode_contained(
      &mut pkgs_aux.get_slot_leaders().data(slot, 2).build(),
      pkgs_aux,
    )
    .await
    .unwrap()
    .result
    .unwrap();
});

create_http_test!(&mut *pool_resource().await, http(), get_slot, |pkgs_aux, trans| async {
  let _res = trans
    .send_retrieve_and_decode_contained(&mut pkgs_aux.get_slot().data(None).build(), pkgs_aux)
    .await
    .unwrap();
});

create_http_test!(&mut *pool_resource().await, http(), get_supply, |pkgs_aux, trans| async {
  let _res = trans
    .send_retrieve_and_decode_contained(&mut pkgs_aux.get_supply().data(None).build(), pkgs_aux)
    .await
    .unwrap()
    .result
    .unwrap();
});

create_http_test!(
  &mut *pool_resource().await,
  http(),
  get_token_account_balance,
  |pkgs_aux, trans| async {
    let _res = trans
      .send_retrieve_and_decode_contained(
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
  &mut *pool_resource().await,
  http(),
  get_token_accounts_by_delegate,
  |pkgs_aux, trans| async {
    let _res = trans
      .send_retrieve_and_decode_contained(
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
  &mut *pool_resource().await,
  http(),
  get_token_accounts_by_owner,
  |pkgs_aux, trans| async {
    let _res = trans
      .send_retrieve_and_decode_contained(
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
  &mut *pool_resource().await,
  http(),
  get_token_largest_accounts,
  |pkgs_aux, trans| async {
    let _res = trans
      .send_retrieve_and_decode_contained(
        &mut pkgs_aux.get_token_largest_accounts().data(TO_SOL_TOKEN_MINT, None).build(),
        pkgs_aux,
      )
      .await
      .unwrap()
      .result
      .unwrap();
  }
);

create_http_test!(&mut *pool_resource().await, http(), get_token_supply, |pkgs_aux, trans| async {
  let _res = trans
    .send_retrieve_and_decode_contained(
      &mut pkgs_aux.get_token_supply().data(TO_SOL_TOKEN_MINT, None).build(),
      pkgs_aux,
    )
    .await
    .unwrap()
    .result
    .unwrap();
});

create_http_test!(
  &mut *pool_resource().await,
  http(),
  get_transaction_count,
  |pkgs_aux, trans| async {
    let _res = trans
      .send_retrieve_and_decode_contained(
        &mut pkgs_aux.get_transaction_count().data(None).build(),
        pkgs_aux,
      )
      .await
      .unwrap()
      .result
      .unwrap();
  }
);

create_http_test!(&mut *pool_resource().await, http(), get_version, |pkgs_aux, trans| async {
  let _res = trans
    .send_retrieve_and_decode_contained(&mut pkgs_aux.get_version().build(), pkgs_aux)
    .await
    .unwrap()
    .result
    .unwrap()
    .feature_set;
});

create_http_test!(
  &mut *pool_resource().await,
  http(),
  get_vote_accounts,
  |pkgs_aux, trans| async {
    let config: Option<GetVoteAccountsConfig<&str>> = None;
    let _res = trans
      .send_retrieve_and_decode_contained(
        &mut pkgs_aux.get_vote_accounts().data(config).build(),
        pkgs_aux,
      )
      .await
      .unwrap()
      .result
      .unwrap();
  }
);

create_http_test!(
  &mut *pool_resource().await,
  http(),
  is_blockhash_valid,
  |pkgs_aux, trans| async {
    let _res = trans
      .send_retrieve_and_decode_contained(
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
  &mut *pool_resource().await,
  http(),
  minimum_ledger_slot,
  |pkgs_aux, trans| async {
    let _res = trans
      .send_retrieve_and_decode_contained(&mut pkgs_aux.minimum_ledger_slot().build(), pkgs_aux)
      .await
      .unwrap()
      .result
      .unwrap();
  }
);

#[cfg(all(feature = "ed25519-dalek", feature = "solana-program"))]
create_http_test!(
  &mut *pool_resource().await,
  http(),
  http_get_latest_blockhash_send_transaction_and_get_transaction,
  |pkgs_aux, trans| async {
    let from_keypair = ed25519_dalek::Keypair::from_bytes(&alice_keypair()[..]).unwrap();
    let blockhash = latest_blockhash(pkgs_aux, trans).await;
    let tx = crate::blockchain::solana::TransactionInput::new(
      &mut pkgs_aux.byte_buffer,
      blockhash,
      transfer_message(blockhash, from_keypair.public.to_bytes()).into(),
      &[from_keypair],
    )
    .unwrap();
    let tx_hash = trans
      .send_retrieve_and_decode_contained(
        &mut pkgs_aux.send_transaction().data(None, &tx).unwrap().build(),
        pkgs_aux,
      )
      .await
      .unwrap()
      .result
      .unwrap();
    Solana::confirm_transaction(
      <_>::default(),
      &mut (&mut *pkgs_aux, &mut *trans).into(),
      &tx_hash,
    )
    .await
    .unwrap();

    let _res = trans
      .send_retrieve_and_decode_contained(
        &mut pkgs_aux
          .get_transaction()
          .data(
            tx_hash.as_str(),
            Some(crate::blockchain::solana::GetTransactionConfig {
              commitment: Some(Commitment::Finalized),
              encoding: Some(TransactionEncoding::Base64),
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
  &mut *pool_resource().await,
  http(),
  http_reqs_with_array,
  |pkgs_aux, trans| async {
    let mut buffer = Vec::new();
    let _res = trans
      .send_retrieve_and_decode_batch(
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
  WS_URL,
  &mut *pool_resource().await,
  ws(),
  account_subscribe,
  (account_unsubscribe),
  |pkgs_aux, trans| async {
    [trans
      .send_retrieve_and_decode_contained(
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
  WS_URL,
  &mut *pool_resource().await,
  ws(),
  root_subscribe,
  (root_unsubscribe),
  |pkgs_aux, trans| async {
    [trans
      .send_retrieve_and_decode_contained(&mut pkgs_aux.root_subscribe().build(), pkgs_aux)
      .await
      .unwrap()
      .result
      .unwrap()]
  }
);

create_ws_test!(
  WS_URL,
  &mut *pool_resource().await,
  ws(),
  slot_subscribe,
  (slot_unsubscribe),
  |pkgs_aux, trans| async {
    [trans
      .send_retrieve_and_decode_contained(&mut pkgs_aux.slot_subscribe().build(), pkgs_aux)
      .await
      .unwrap()
      .result
      .unwrap()]
  }
);

create_ws_test!(
  WS_URL,
  &mut *pool_resource().await,
  ws(),
  slot_updates_subscribe,
  (slots_updates_unsubscribe),
  |pkgs_aux, trans| async {
    [trans
      .send_retrieve_and_decode_contained(&mut pkgs_aux.slots_updates_subscribe().build(), pkgs_aux)
      .await
      .unwrap()
      .result
      .unwrap()]
  }
);

create_ws_test!(
  WS_URL,
  &mut *pool_resource().await,
  ws(),
  ws_reqs_with_array,
  (account_unsubscribe, account_unsubscribe),
  |pkgs_aux, trans| async {
    let mut buffer = Vec::new();
    trans
      .send_retrieve_and_decode_batch(
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

fn alice_keypair() -> [u8; 64] {
  let mut array = [0; 64];
  array[..32].copy_from_slice(&alice_sk());
  array[32..].copy_from_slice(&alice_pk());
  array
}

fn alice_pk() -> SolanaAddressHash {
  let mut buffer = SolanaAddressHash::default();
  for (s, b) in std::env::var("ALICE_PK").unwrap().split(',').zip(&mut buffer) {
    *b = s.parse().unwrap();
  }
  buffer
}

fn alice_sk() -> SolanaAddressHash {
  let mut buffer = SolanaAddressHash::default();
  for (s, b) in std::env::var("ALICE_SK").unwrap().split(',').zip(&mut buffer) {
    *b = s.parse().unwrap();
  }
  buffer
}

fn bob_pk() -> SolanaAddressHash {
  [
    24, 147, 209, 196, 197, 185, 156, 48, 170, 96, 192, 119, 193, 150, 129, 12, 221, 102, 119, 84,
    33, 221, 67, 224, 185, 107, 130, 157, 207, 85, 161, 30,
  ]
}

fn http() -> (SerdeJson, HttpParams) {
  (SerdeJson, HttpParams::from_uri("https://api.testnet.solana.com"))
}

async fn latest_blockhash(
  pkgs_aux: &mut SolanaMutPkgsAux<'_, SerdeJson, HttpParams>,
  trans: &mut Client,
) -> SolanaAddressHash {
  trans
    .send_retrieve_and_decode_contained(
      &mut pkgs_aux.get_latest_blockhash().data(None).build(),
      pkgs_aux,
    )
    .await
    .unwrap()
    .result
    .unwrap()
    .value
    .blockhash
}

async fn pool_resource() -> MappedMutexGuard<'static, Solana> {
  type Rm = SimpleRM<crate::Error, (), Solana>;

  static POOL: OnceLock<StaticPool<Mutex<Option<Solana>>, Rm, 8>> = OnceLock::new();

  fn cb(_: &()) -> crate::Result<Solana> {
    Ok(Solana::new(None))
  }

  POOL.get_or_init(|| StaticPool::new(SimpleRM::new(cb, ())).unwrap()).get().await.unwrap()
}

async fn slot(
  pkgs_aux: &mut SolanaMutPkgsAux<'_, SerdeJson, HttpParams>,
  trans: &mut Client,
) -> u64 {
  trans
    .send_retrieve_and_decode_contained(&mut pkgs_aux.get_slot().data(None).build(), pkgs_aux)
    .await
    .unwrap()
    .result
    .unwrap()
}

#[cfg(all(feature = "ed25519-dalek", feature = "solana-program"))]
fn transfer_message(
  blockhash: [u8; 32],
  from_public_key: [u8; 32],
) -> crate::blockchain::solana::MessageInput {
  let transfer =
    solana_program::system_instruction::transfer(&from_public_key.into(), &bob_pk().into(), 0)
      .try_into()
      .unwrap();
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
