/// Utility for generic tests
#[macro_export]
macro_rules! create_generic_test {
  (
    #[$($meta:meta)?],
    $api:expr,
    $drsr_exp:expr,
    $test:ident,
    |$parts_cb_pkgs_aux:ident, $parts_cb_trans:ident| $parts_cb:expr,
    |$rslt_cb_pkgs_aux:ident, $rslt_cb_trans:ident, $rslt_cb_parts:ident| $rslt_cb:expr,
    $trans:expr
  ) => {
    $(#[$meta])?
    #[test]
    fn $test() {
      $crate::tests::_RUNTIME.block_on(async {
        let _tracing = wtx::misc::tracing_tree_init(None);
        let api = $api;
        let (drsr, ext_req_params) = $drsr_exp;
        let mut pair = wtx::client_api_framework::misc::Pair::new(
          PkgsAux::from_minimum(api, drsr, ext_req_params),
          $trans,
        );
        let (pkgs_aux, trans) = pair.parts_mut();
        let $parts_cb_pkgs_aux = pkgs_aux;
        let $parts_cb_trans = trans;

        let $rslt_cb_parts = $parts_cb.await;

        let $rslt_cb_pkgs_aux = $parts_cb_pkgs_aux;
        let $rslt_cb_trans = $parts_cb_trans;
        $rslt_cb.await;
      });
    }
  };
}

/// Utility for HTTP tests
#[macro_export]
macro_rules! create_http_test {
  (
    #[$($meta:meta)?],
    $api:expr,
    $drsr_exp:expr,
    $test:ident,
    $client:expr,
    |$parts_cb_pkgs_aux:ident, $parts_cb_trans:ident| $parts_cb:expr
  ) => {
    $crate::create_generic_test! {
      #[$($meta)?],
      $api,
      $drsr_exp,
      $test,
      |$parts_cb_pkgs_aux, $parts_cb_trans| $parts_cb,
      |_pkgs_aux, _trans, _rslt| async {},
      $client
    }
  };
}

/// Utility for `WebSocket` tests
#[macro_export]
macro_rules! create_ws_test {
  (
    #[$($meta:meta)?],
    $uri:expr,
    $api:expr,
    $drsr_exp:expr,
    $test:ident,
    |$parts_cb_pkgs_aux:ident, $parts_cb_trans:ident| $parts_cb:expr
  ) => {
    $crate::create_generic_test! {
      #[$($meta)?],
      $api,
      $drsr_exp,
      $test,
      |$parts_cb_pkgs_aux, $parts_cb_trans| $parts_cb,
      |_pkgs_aux, _trans, _subs| async {},
      {
        let uri = wtx::misc::Uri::new($uri);
        wtx::web_socket::WebSocketConnector::default()
        .connect(
          wtx::misc::TokioRustlsConnector::from_auto()
            .unwrap()
            .connect_without_client_auth(
              uri.hostname(),
              tokio::net::TcpStream::connect(uri.hostname_with_implied_port()).await.unwrap()
            )
            .await
            .unwrap(),
          &uri
        )
        .await
        .unwrap()
      }
    }
  };
}

/// Makes successive HTTP requests over a period defined in `cto` until the transaction is
/// successful or expired.
#[cfg(feature = "solana")]
#[macro_export]
macro_rules! confirm_solana_tx {
  ($cto:expr, $pair:expr, $tx_hash:expr $(,)?) => {
    async move {
      match $cto {
        $crate::blockchain::ConfirmTransactionOptions::Tries { number } => {
          for _ in 0u16..number {
            if $crate::blockchain::solana::Solana::check_confirmation($pair, $tx_hash).await? {
              return Ok(());
            }
          }
        }
        $crate::blockchain::ConfirmTransactionOptions::TriesWithInterval { interval, number } => {
          let mut iter = 0u16..number;
          if let Some(_) = iter.next() {
            if $crate::blockchain::solana::Solana::check_confirmation($pair, $tx_hash).await? {
              return Ok(());
            }
          }
          for _ in iter {
            wtx::misc::sleep(interval).await?;
            if $crate::blockchain::solana::Solana::check_confirmation($pair, $tx_hash).await? {
              return Ok(());
            }
          }
        }
      }

      Err($crate::Error::CouldNotConfirmTransaction)
    }
  };
}

macro_rules! _create_blockchain_constants {
  (
    $address_hash_vis:vis address_hash: $address_hash:ident = $_1:literal,
    $address_hash_str_vis:vis address_hash_str: $address_hash_str:ident = $_2:literal,
    $block_hash_vis:vis block_hash: $block_hash:ident = $_3:literal,
    $block_hash_str_vis:vis block_hash_str: $block_hash_str:ident = $_4:literal,
    $signature_hash_vis:vis signature_hash: $signature_hash:ident = $_5:literal,
    $signature_hash_str_vis:vis signature_hash_str: $signature_hash_str:ident = $_6:literal,
    $transaction_hash_vis:vis transaction_hash: $transaction_hash:ident = $_7:literal,
    $transaction_hash_str_vis:vis transaction_hash_str: $transaction_hash_str:ident = $_8:literal
  ) => {
    /// Address hash as bytes
    $address_hash_vis type $address_hash = [u8; $_1];
    /// Address hash as an encoded string
    $address_hash_str_vis type $address_hash_str = ::wtx::collection::ArrayStringU8<$_2>;

    /// Block hash as bytes
    $block_hash_vis type $block_hash = [u8; $_3];
    /// Block hash as an encoded string
    $block_hash_str_vis type $block_hash_str = ::wtx::collection::ArrayStringU8<$_4>;

    /// Signature hash as bytes
    $signature_hash_vis type $signature_hash = ::wtx::collection::ArrayWrapper<u8, $_5>;
    /// Signature hash as an encoded string
    $signature_hash_str_vis type $signature_hash_str = ::wtx::collection::ArrayStringU8<$_6>;

    /// Transaction hash as bytes
    $transaction_hash_vis type $transaction_hash = ::wtx::collection::ArrayWrapper<u8, $_7>;
    /// Transaction hash as an encoded string
    $transaction_hash_str_vis type $transaction_hash_str = ::wtx::collection::ArrayStringU8<$_8>;
  };
}

macro_rules! _generic_api_doc {
  () => {
    "Used to group a set of packages related to this API as well as any additional instance parameters."
  };
}

macro_rules! _generic_res_data_elem_doc {
  () => {
    "Element that makes up most of the expected data response returned by the server."
  };
}
