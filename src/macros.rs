/// Utility for generic tests
#[macro_export]
macro_rules! create_generic_test {
  ($executor:ident, $test:ident, $pair:expr, $parts_cb:expr, $rslt_cb:expr) => {
    #[$executor::test]
    async fn $test() {
      fn parts_cb_infer<'pair, API, DRSR, O, T>(
        pkgs_aux: &'pair mut PkgsAux<API, DRSR, T::Params>,
        trans: &'pair mut T,
        cb: impl FnOnce(&'pair mut PkgsAux<API, DRSR, T::Params>, &'pair mut T) -> O,
      ) -> O
      where
        T: Transport<DRSR>,
      {
        cb(pkgs_aux, trans)
      }
      fn rslt_cb_infer<'pair, API, DRSR, O, R, T>(
        pkgs_aux: &'pair mut PkgsAux<API, DRSR, T::Params>,
        trans: &'pair mut T,
        rslt: R,
        cb: impl FnOnce(&'pair mut PkgsAux<API, DRSR, T::Params>, &'pair mut T, R) -> O,
      ) -> O
      where
        T: Transport<DRSR>,
      {
        cb(pkgs_aux, trans, rslt)
      }
      init_test_cfg();
      let mut pair = $pair;
      let (pkg, pkgs_aux) = pair.parts_mut();
      let rslt = parts_cb_infer(pkg, pkgs_aux, $parts_cb).await;
      rslt_cb_infer(pkg, pkgs_aux, rslt, $rslt_cb).await;
    }
  };
}

/// Utility for HTTP tests
#[macro_export]
macro_rules! create_http_test {
  ($api:expr, $drsr_erp:expr, $test:ident, $cb:expr) => {
    $crate::create_generic_test! {
      tokio,
      $test,
      {
        let (drsr, ext_req_params) = $drsr_erp;
        wtx::client_api_framework::misc::Pair::new(
          PkgsAux::from_minimum($api, drsr, ext_req_params),
          reqwest::Client::default()
        )
      },
      $cb,
      |_, _, _| async {}
    }
  };
}

/// Utility for WebSocket tests
#[macro_export]
macro_rules! create_ws_test {
  (
    $url:expr,
    $api:expr,
    $drsr_erp:expr,
    $sub:ident,
    ($($unsub:ident),+),
    $cb:expr
  ) => {
    $crate::create_generic_test! {
      tokio,
      $sub,
      {
        use wtx::web_socket::handshake::WebSocketConnect;
        let uri_parts = wtx::misc::UriParts::from($url);
        let (drsr, ext_req_params) = $drsr_erp;
        let mut fb = wtx::web_socket::FrameBufferVec::default();
        let trans = wtx::web_socket::handshake::WebSocketConnectRaw {
          compression: (),
          fb: &mut fb,
          headers_buffer: &mut <_>::default(),
          rng: wtx::rng::StaticRng::default(),
          stream: tokio::net::TcpStream::connect(uri_parts.host()).await.unwrap(),
          uri: $url,
          wsb: wtx::web_socket::WebSocketBuffer::default(),
        }
        .connect()
        .await
        .unwrap()
        .1;
        wtx::client_api_framework::misc::Pair::new(
          PkgsAux::from_minimum($api, drsr, ext_req_params),
          (fb, trans)
        )
      },
      $cb,
      |pkgs_aux, trans, subs| async move {
        let mut iter = subs.into_iter();
        let ids = &mut [$( pkgs_aux.$unsub().data(iter.next().unwrap()).build(), )+][..];
        let _res = trans.send(&mut wtx::client_api_framework::pkg::BatchPkg::new(ids), pkgs_aux).await.unwrap();
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
    $address_hash_str_vis type $address_hash_str = ::arrayvec::ArrayString<$_2>;

    /// Block hash as bytes
    $block_hash_vis type $block_hash = [u8; $_3];
    /// Block hash as an encoded string
    $block_hash_str_vis type $block_hash_str = ::arrayvec::ArrayString<$_4>;

    /// Signature hash as bytes
    $signature_hash_vis type $signature_hash = ::cl_aux::ArrayWrapper<u8, $_5>;
    /// Signature hash as an encoded string
    $signature_hash_str_vis type $signature_hash_str = ::arrayvec::ArrayString<$_6>;

    /// Transaction hash as bytes
    $transaction_hash_vis type $transaction_hash = ::cl_aux::ArrayWrapper<u8, $_7>;
    /// Transaction hash as an encoded string
    $transaction_hash_str_vis type $transaction_hash_str = ::arrayvec::ArrayString<$_8>;
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
