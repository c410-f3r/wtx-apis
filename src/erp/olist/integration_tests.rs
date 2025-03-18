use crate::erp::olist::{
  ACC_PROD_URI, API_PROD_URI, ContactPost, Olist, OrderPost, PersonTy, PkgsAux, Plan,
};
use alloc::string::ToString;
use core::time::Duration;
use std::sync::LazyLock;
use tokio::sync::Mutex;
use wtx::{
  client_api_framework::network::{HttpParams, transport::SendingReceivingTransport},
  data_transformation::dnsn::SerdeJson,
  http::client_pool::{ClientPoolBuilder, ClientPoolTokioRustls},
  misc::{Uri, Vector},
};

static CLIENT_ACC: LazyLock<ClientPoolTokioRustls<fn()>> =
  LazyLock::new(|| ClientPoolBuilder::tokio_rustls(1).build());
static CLIENT_API: LazyLock<ClientPoolTokioRustls<fn()>> =
  LazyLock::new(|| ClientPoolBuilder::tokio_rustls(1).build());
static OLIST: LazyLock<Mutex<Olist>> = LazyLock::new(|| {
  let client_id = std::env::var("OLIST_CLIENT_ID").unwrap();
  let client_secret = std::env::var("OLIST_CLIENT_SECRET").unwrap();
  let refresh_token = std::env::var("OLIST_REFRESH_TOKEN").unwrap();
  Mutex::new(
    Olist::new(
      client_id,
      client_secret,
      60,
      Plan::Crescer,
      refresh_token.as_str().try_into().unwrap(),
    )
    .unwrap(),
  )
});

create_http_test!(
  #[ignore],
  &mut *OLIST.lock().await,
  http(),
  get_info,
  &*CLIENT_API,
  |pkgs_aux, trans| async {
    refresh_token(&mut pkgs_aux.api).await;
    let _res = trans
      .send_pkg_recv_decode_contained(&mut pkgs_aux.get_info().build(), pkgs_aux)
      .await
      .unwrap()
      .data;
  }
);

create_http_test!(
  #[ignore],
  &mut *OLIST.lock().await,
  http(),
  post_contact,
  &*CLIENT_API,
  |pkgs_aux, trans| async {
    refresh_token(&mut pkgs_aux.api).await;

    let _res = trans
      .send_pkg_recv_decode_contained(
        &mut pkgs_aux
          .post_contact()
          .data(ContactPost {
            nome: "Test user",
            codigo: None,
            fantasia: None,
            tipo_pessoa: PersonTy::Natural,
            cpf_cnpj: None,
            inscricao_estadual: None,
            rg: None,
            telefone: None,
            celular: None,
            email: None,
            endereco: None,
            endereco_cobranca: None,
            inscricao_municipal: None,
            telefone_adicional: None,
            email_nfe: None,
            site: None,
            regime_tributario: None,
            estado_civil: None,
            profissao: None,
            sexo: None,
            data_nascimento: None,
            naturalidade: None,
            nome_pai: None,
            nome_mae: None,
            cpf_pai: None,
            cpf_mae: None,
            limite_credito: None,
            situacao: None,
            observacoes: None,
            vendedor: None,
            tipos: None,
            contatos: None,
          })
          .build(),
        pkgs_aux,
      )
      .await
      .unwrap()
      .data
      .result
      .unwrap();
  }
);

create_http_test!(
  #[ignore],
  &mut *OLIST.lock().await,
  http(),
  post_and_get_order,
  &*CLIENT_API,
  |pkgs_aux, trans| async {
    refresh_token(&mut pkgs_aux.api).await;
    let numero_pedido = trans
      .send_pkg_recv_decode_contained(
        &mut pkgs_aux
          .post_order()
          .data(OrderPost::<&str> {
            data_prevista: Some(chrono::NaiveDate::from_ymd_opt(2025, 1, 25).unwrap()),
            data_envio: None,
            observacoes: None,
            observacoes_internas: None,
            situacao: None,
            data: Some(chrono::NaiveDate::from_ymd_opt(2025, 1, 25).unwrap()),
            data_entrega: None,
            numero_ordem_compra: None,
            valor_desconto: None,
            valor_frete: None,
            valor_outras_despesas: None,
            id_contato: 757850497,
            lista_preco: None,
            natureza_operacao: None,
            vendedor: None,
            endereco_entrega: None,
            ecommerce: None,
            transportador: None,
            intermediador: None,
            deposito: None,
            pagamento: None,
            itens: None,
          })
          .build(),
        pkgs_aux,
      )
      .await
      .unwrap()
      .data
      .result
      .unwrap()
      .id
      .to_string();
    tokio::time::sleep(Duration::from_secs(5)).await;
    let _res = trans
      .send_pkg_recv_decode_contained(
        &mut pkgs_aux.get_order().params(&numero_pedido).build(),
        pkgs_aux,
      )
      .await
      .unwrap()
      .data
      .result
      .unwrap();
  }
);

fn http() -> (SerdeJson, HttpParams) {
  (SerdeJson, HttpParams::from_uri(API_PROD_URI.into()))
}

async fn refresh_token(olist: &mut Olist) {
  olist
    .sync()
    .request_tokens(
      (
        &mut SerdeJson,
        &mut CLIENT_ACC.lock(&Uri::new(ACC_PROD_URI)).await.unwrap().client,
        &mut HttpParams::from_uri(ACC_PROD_URI.into()),
      ),
      &mut Vector::new(),
    )
    .await
    .unwrap();
}
