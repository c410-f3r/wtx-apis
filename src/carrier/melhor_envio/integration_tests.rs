use crate::carrier::melhor_envio::{
  MelhorEnvio, PROD_URI, PkgsAux,
  calculate_shipment_request::{
    CalculateShipmentRequest, CalculateShipmentRequestPostalCode, CalculateShipmentRequestProduct,
  },
};
use rust_decimal_macros::dec;
use std::sync::LazyLock;
use tokio::sync::Mutex;
use wtx::{
  client_api_framework::network::{HttpParams, transport::SendingReceivingTransport},
  collection::Vector,
  data_transformation::dnsn::SerdeJson,
  http::client_pool::{ClientPoolBuilder, ClientPoolTokioRustls},
};

const DEST_ZIPCODE: &str = "04571000";
const ORIGIN_ZIPCODE: &str = "01310930";
const PRODUCTS: [CalculateShipmentRequestProduct<&'static str>; 2] = [
  CalculateShipmentRequestProduct {
    id: "1",
    width: dec!(15.0),
    height: dec!(3.0),
    length: dec!(14.0),
    weight: dec!(0.3),
    insurance_value: dec!(0),
    quantity: dec!(2.0),
  },
  CalculateShipmentRequestProduct {
    id: "2",
    width: dec!(18.0),
    height: dec!(5.0),
    length: dec!(20.0),
    weight: dec!(0.6),
    insurance_value: dec!(0),
    quantity: dec!(3.0),
  },
];

static CLIENT: LazyLock<ClientPoolTokioRustls<fn(&()), (), ()>> =
  LazyLock::new(|| ClientPoolBuilder::tokio_rustls(1).build());
static SUPER_FRETE: LazyLock<Mutex<MelhorEnvio>> = LazyLock::new(|| {
  let client_id = std::env::var("MELHOR_ENVIO_CLIENT_ID").unwrap();
  let client_secret = std::env::var("MELHOR_ENVIO_CLIENT_SECRET").unwrap();
  let refresh_token = std::env::var("MELHOR_ENVIO_REFRESH_TOKEN").unwrap();
  Mutex::new(
    MelhorEnvio::new(client_id, client_secret, 60, refresh_token.as_str().try_into().unwrap())
      .unwrap(),
  )
});

create_http_test!(
  #[ignore],
  &mut *SUPER_FRETE.lock().await,
  http(),
  quote_send_and_cancel_freight,
  &*CLIENT,
  |pkgs_aux, trans| async {
    let _quotes = trans
      .send_pkg_recv_decode_contained(
        &mut pkgs_aux
          .calculate_shipment()
          .data(CalculateShipmentRequest {
            from: CalculateShipmentRequestPostalCode { postal_code: ORIGIN_ZIPCODE },
            to: CalculateShipmentRequestPostalCode { postal_code: DEST_ZIPCODE },
            products: Some(Vector::from_iter(PRODUCTS).unwrap()),
            options: None,
            volumes: None,
          })
          .build(),
        pkgs_aux,
      )
      .await
      .unwrap()
      .data
      .into_rslt()
      .unwrap();
  }
);

fn http() -> (SerdeJson, HttpParams) {
  (SerdeJson, HttpParams::from_uri(PROD_URI.into()))
}
