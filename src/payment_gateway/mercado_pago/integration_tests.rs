use crate::payment_gateway::mercado_pago::{
  ExcludedPaymentType, Item, MercadoPago, PROD_URI, PaymentMethods, PaymentTypeId, PkgsAux,
  Preference,
};
use rust_decimal::Decimal;
use std::sync::LazyLock;
use tokio::sync::Mutex;
use wtx::{
  client_api_framework::network::{HttpParams, transport::SendingReceivingTransport},
  data_transformation::dnsn::SerdeJson,
  http::client_pool::{ClientPoolBuilder, ClientPoolTokioRustls},
  misc::ArrayVector,
};

static CLIENT: LazyLock<ClientPoolTokioRustls<fn()>> =
  LazyLock::new(|| ClientPoolBuilder::tokio_rustls(1).build());
static MERCADO_PAGO: LazyLock<Mutex<MercadoPago>> = LazyLock::new(|| {
  let client_id = std::env::var("MERCADO_PAGO_CLIENT_ID").unwrap();
  let client_secret = std::env::var("MERCADO_PAGO_CLIENT_SECRET").unwrap();
  Mutex::new(MercadoPago::new(client_id, client_secret, 60, true))
});

create_http_test!(
  #[ignore],
  &mut *MERCADO_PAGO.lock().await,
  http(),
  create_preference,
  &*CLIENT,
  |pkgs_aux, trans| async {
    let _rslt = trans
      .send_pkg_recv_decode_contained(
        &mut pkgs_aux
          .post_preference()
          .data(Preference {
            additional_info: None,
            auto_return: None,
            back_urls: None,
            differential_pricing: None,
            expires: None,
            expiration_date_from: None,
            expiration_date_to: None,
            external_reference: None,
            items: wtx::vector![Item {
              id: "1",
              title: "Test title",
              description: Some("Test description"),
              picture_url: None,
              category_id: None,
              quantity: Decimal::TEN,
              currency_id: None,
              unit_price: Decimal::PI,
            }],
            marketplace: None,
            marketplace_fee: None,
            notification_url: None,
            payer: None,
            payment_methods: Some(PaymentMethods {
              default_installments: None,
              default_payment_method_id: None,
              excluded_payment_methods: None,
              excluded_payment_types: Some(
                ArrayVector::from_iter([ExcludedPaymentType { id: PaymentTypeId::Ticket }])
                  .unwrap(),
              ),
              installments: Some(12),
            }),
            shipments: None,
            tracks: None,
          })
          .build(),
        pkgs_aux,
      )
      .await
      .unwrap()
      .data;
  }
);

create_http_test!(
  #[ignore],
  &mut *MERCADO_PAGO.lock().await,
  http(),
  read_payment,
  &*CLIENT,
  |pkgs_aux, trans| async {
    let _rslt = trans
      .send_pkg_recv_decode_contained(
        &mut pkgs_aux.get_payment().params(1330392301).build(),
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
