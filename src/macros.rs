/// Utility for generic tests
#[macro_export]
macro_rules! create_generic_test {
  (
    $api:expr,
    $drsr_exp:expr,
    $test:ident,
    |$parts_cb_pkgs_aux:ident, $parts_cb_trans:ident| $parts_cb:expr,
    |$rslt_cb_pkgs_aux:ident, $rslt_cb_trans:ident, $rslt_cb_parts:ident| $rslt_cb:expr,
    $trans:expr
  ) => {
    #[test]
    fn $test() {
      $crate::tests::_RUNTIME.block_on(async {
        #[cfg(test)]
        $crate::misc::init_test_cfg();
        #[cfg(test)]
        let _path = dotenv::dotenv();
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
    $api:expr,
    $drsr_exp:expr,
    $test:ident,
    $client:expr,
    |$parts_cb_pkgs_aux:ident, $parts_cb_trans:ident| $parts_cb:expr
  ) => {
    $crate::create_generic_test! {
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
    $uri:expr,
    $api:expr,
    $drsr_exp:expr,
    $test:ident,
    ($($unsub:ident),+),
    |$parts_cb_pkgs_aux:ident, $parts_cb_trans:ident| $parts_cb:expr
  ) => {
    $crate::create_generic_test! {
      $api,
      $drsr_exp,
      $test,
      |$parts_cb_pkgs_aux, $parts_cb_trans| $parts_cb,
      |pkgs_aux, trans, subs| async move {
        let mut iter = subs.into_iter();
        let ids = &mut [$( pkgs_aux.$unsub().data(iter.next().unwrap()).build(), )+][..];
        let _res = trans.send(&mut wtx::client_api_framework::pkg::BatchPkg::new(ids), pkgs_aux).await.unwrap();
      },
      {
        use wtx::web_socket::handshake::WebSocketConnect;
        let uri = wtx::misc::Uri::new($uri);
        let mut fb = wtx::web_socket::FrameBufferVec::default();
        let trans = wtx::web_socket::handshake::WebSocketConnectRaw {
          compression: (),
          fb: &mut fb,
          headers_buffer: &mut wtx::web_socket::handshake::HeadersBuffer::default(),
          rng: wtx::rng::StaticRng::default(),
          stream: wtx::misc::TokioRustlsConnector::from_webpki_roots()
            .with_tcp_stream(uri.host(), uri.hostname())
            .await
            .unwrap(),
          uri: &uri,
          wsb: wtx::web_socket::WebSocketBuffer::default(),
        }
        .connect([])
        .await
        .unwrap()
        .1;
      (fb, trans)
      }
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
    $address_hash_str_vis type $address_hash_str = ::wtx::misc::ArrayString<$_2>;

    /// Block hash as bytes
    $block_hash_vis type $block_hash = [u8; $_3];
    /// Block hash as an encoded string
    $block_hash_str_vis type $block_hash_str = ::wtx::misc::ArrayString<$_4>;

    /// Signature hash as bytes
    $signature_hash_vis type $signature_hash = ::cl_aux::ArrayWrapper<u8, $_5>;
    /// Signature hash as an encoded string
    $signature_hash_str_vis type $signature_hash_str = ::wtx::misc::ArrayString<$_6>;

    /// Transaction hash as bytes
    $transaction_hash_vis type $transaction_hash = ::cl_aux::ArrayWrapper<u8, $_7>;
    /// Transaction hash as an encoded string
    $transaction_hash_str_vis type $transaction_hash_str = ::wtx::misc::ArrayString<$_8>;
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
