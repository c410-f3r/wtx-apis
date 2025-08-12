use crate::carrier::super_frete::{
  CancelOrderReqOrder, PROD_URI, PkgsAux, QuoteFreightReq, QuoteFreightReqPostalCode,
  QuoteFreightReqProduct, SendFreightReq, SendFreightReqAddress, SendFreightReqAddressTo,
  SendFreightReqOptions, SendFreightReqProduct, SendFreightReqVolumes, SuperFrete,
};
use alloc::string::String;
use rust_decimal_macros::dec;
use std::sync::LazyLock;
use tokio::sync::Mutex;
use wtx::{
  client_api_framework::network::{HttpParams, transport::SendingReceivingTransport},
  collection::Vector,
  de::format::SerdeJson,
  http::client_pool::{ClientPoolBuilder, ClientPoolTokioRustls},
  misc::Wrapper,
};

const DEST_ZIPCODE: &str = "04571000";
const ORIGIN_ZIPCODE: &str = "01310930";
static PRODUCTS: [QuoteFreightReqProduct; 2] = [
  QuoteFreightReqProduct {
    height: dec!(4.0),
    length: dec!(4.0),
    quantity: dec!(2.0),
    weight: dec!(0.3),
    width: dec!(4.0),
  },
  QuoteFreightReqProduct {
    height: dec!(5.0),
    length: dec!(5.0),
    quantity: dec!(3.0),
    weight: dec!(0.2),
    width: dec!(5.0),
  },
];

static CLIENT: LazyLock<ClientPoolTokioRustls<fn(&()), (), ()>> =
  LazyLock::new(|| ClientPoolBuilder::tokio_rustls(1).build());
static SUPER_FRETE: LazyLock<Mutex<SuperFrete>> = LazyLock::new(|| {
  let token = std::env::var("SUPER_FRETE_TOKEN").unwrap();
  Mutex::new(SuperFrete::new(token))
});

create_http_test!(
  #[ignore],
  &mut *SUPER_FRETE.lock().await,
  http(),
  quote_send_and_cancel_freight,
  &*CLIENT,
  |pkgs_aux, trans| async {
    let quotes = trans
      .send_pkg_recv_decode_contained(
        &mut pkgs_aux
          .quote_freight()
          .data(QuoteFreightReq {
            from: QuoteFreightReqPostalCode { postal_code: ORIGIN_ZIPCODE },
            options: None,
            package: None,
            products: Some(&PRODUCTS),
            services: "1,2,3,4,17",
            to: QuoteFreightReqPostalCode { postal_code: DEST_ZIPCODE },
          })
          .build(),
        pkgs_aux,
      )
      .await
      .unwrap()
      .data
      .into_rslt()
      .unwrap();

    let products = PRODUCTS
      .iter()
      .map(|el| SendFreightReqProduct {
        name: "Product",
        quantity: el.quantity,
        unitary_value: dec!(20),
      })
      .collect::<Wrapper<Result<Vector<_>, _>>>()
      .0
      .unwrap();
    let quote = &quotes[0];
    let service = quote.id;
    let package = quote.packages.iter().next().unwrap();
    let volumes = SendFreightReqVolumes {
      height: package.dimensions.height,
      length: package.dimensions.length,
      weight: package.weight,
      width: package.dimensions.width,
    };

    let id: String = trans
      .send_pkg_recv_decode_contained(
        &mut pkgs_aux
          .send_freight()
          .data(SendFreightReq {
            from: SendFreightReqAddress {
              address: "Avenida Paulista",
              city: "São Paulo",
              complement: None,
              district: "Bela Vista",
              name: "From example",
              number: "2100",
              postal_code: ORIGIN_ZIPCODE,
              state_abbr: "SP",
            },
            options: SendFreightReqOptions {
              insurance_value: None,
              invoice: None,
              non_commercial: None,
              own_hand: None,
              receipt: None,
            },
            platform: None,
            products: Some(&products),
            service,
            to: SendFreightReqAddressTo {
              address: SendFreightReqAddress {
                address: "Avenida Engenheiro Luiz Carlos Berrini",
                city: "São Paulo",
                complement: None,
                district: "Cidade Monções",
                name: "To example",
                number: "1452",
                postal_code: DEST_ZIPCODE,
                state_abbr: "SP",
              },
              email: "example@example.com",
            },
            volumes,
          })
          .build(),
        pkgs_aux,
      )
      .await
      .unwrap()
      .data
      .into_rslt()
      .unwrap()
      .id
      .into();

    let _rslt = trans
      .send_pkg_recv_decode_contained(
        &mut pkgs_aux
          .cancel_order()
          .data(CancelOrderReqOrder { description: "Canceled", id: &id })
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
