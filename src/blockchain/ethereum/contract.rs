//! Ethereum Contract Interface

mod detokenize;
mod options;
mod tokenizable;
mod tokenizable_item;
mod tokenize;

use crate::blockchain::ethereum::{
  BlockId, Bytes, CallRequest, EthCallPkg, EthCallReq, EthEstimateGasReq, EthGetLogsReq,
  EthSendTransactionReq, EthSendTransactionRes, Ethereum, EthereumPkgsAux, FilterBuilder, Log,
  TransactionRequest,
};
pub use detokenize::Detokenize;
use ethabi::Address;
use ethereum_types::{H256, U256};
pub use options::Options;
pub use tokenizable::Tokenizable;
pub use tokenizable_item::TokenizableItem;
pub use tokenize::Tokenize;
use wtx::{
  client_api_framework::{
    misc::Pair,
    network::{HttpParams, transport::SendingReceivingTransport},
    pkg::Package,
  },
  collection::Vector,
  data_transformation::{
    dnsn::De,
    format::{JsonRpcRequest, JsonRpcResponse},
  },
  misc::{DecodeSeq, Encode, Wrapper},
};

/// Ethereum Contract Interface
#[derive(Debug)]
pub struct Contract<DRSR, T>
where
  T: SendingReceivingTransport<HttpParams>,
{
  abi: ethabi::Contract,
  address: Address,
  ethereum: Pair<EthereumPkgsAux<DRSR, HttpParams>, T>,
}

impl<DRSR, T> Contract<DRSR, T>
where
  T: SendingReceivingTransport<HttpParams>,
{
  /// Creates new Contract Interface given blockchain address and ABI
  #[inline]
  pub fn new(
    abi: ethabi::Contract,
    address: Address,
    ethereum: Pair<EthereumPkgsAux<DRSR, HttpParams>, T>,
  ) -> Self {
    Self { abi, address, ethereum }
  }

  /// Creates new Contract Interface given blockchain address and JSON containing ABI
  #[inline]
  pub fn from_json(
    address: Address,
    ethereum: Pair<EthereumPkgsAux<DRSR, HttpParams>, T>,
    json: &[u8],
  ) -> ethabi::Result<Self> {
    Ok(Self::new(ethabi::Contract::load(json)?, address, ethereum))
  }

  /// Get the underlying contract ABI.
  #[inline]
  pub fn abi(&self) -> &ethabi::Contract {
    &self.abi
  }

  /// Returns contract address
  #[inline]
  pub fn address(&self) -> Address {
    self.address
  }

  /// Execute a contract function
  #[inline]
  pub async fn call<FP>(
    &mut self,
    func: &str,
    func_params: FP,
    from: Address,
    options: Options,
  ) -> crate::Result<Option<H256>>
  where
    FP: Tokenize,
    for<'tr> JsonRpcRequest<EthSendTransactionReq<'tr>>: Encode<De<DRSR>>,
    JsonRpcResponse<EthSendTransactionRes>: for<'de> DecodeSeq<'de, De<DRSR>>,
  {
    let data = self.abi.function(func)?.encode_input(&func_params.into_tokens())?.into();
    let Options {
      gas,
      gas_price,
      value,
      nonce,
      condition,
      ty: transaction_type,
      access_list,
      max_fee_per_gas,
      max_priority_fee_per_gas,
    } = options;
    let tr = TransactionRequest {
      from,
      to: Some(self.address),
      gas,
      gas_price,
      value,
      nonce,
      data: Some(Bytes(data)),
      condition,
      ty: transaction_type,
      access_list,
      max_fee_per_gas,
      max_priority_fee_per_gas,
    };
    let (pkgs_aux, trans) = self.ethereum.parts_mut();
    let mut pkg = pkgs_aux.eth_send_transaction().data([&tr]).build();
    Ok(trans.send_pkg_recv_decode_contained(&mut pkg, pkgs_aux).await?.result?)
  }

  /// Estimate gas required for this function call.
  #[inline]
  pub async fn estimate_gas<FP>(
    &mut self,
    func: &str,
    func_params: FP,
    from: Address,
    options: Options,
  ) -> crate::Result<U256>
  where
    FP: Tokenize,
    for<'any> JsonRpcRequest<EthEstimateGasReq<'any>>: Encode<De<DRSR>>,
    JsonRpcResponse<U256>: for<'de> DecodeSeq<'de, De<DRSR>>,
  {
    let data = self.abi.function(func)?.encode_input(&func_params.into_tokens())?.into();
    let call_request = CallRequest {
      from: Some(from),
      to: Some(self.address),
      gas: options.gas,
      gas_price: options.gas_price,
      value: options.value,
      data: Some(Bytes(data)),
      ty: options.ty,
      access_list: options.access_list,
      max_fee_per_gas: options.max_fee_per_gas,
      max_priority_fee_per_gas: options.max_priority_fee_per_gas,
    };
    let (pkgs_aux, trans) = self.ethereum.parts_mut();
    let mut pkg = pkgs_aux.eth_estimate_gas().data(None, &call_request).build();
    Ok(trans.send_pkg_recv_decode_contained(&mut pkg, pkgs_aux).await?.result?)
  }

  /// Find events matching the topics.
  #[inline]
  pub async fn events<AA, BB, CC, R>(
    &mut self,
    event: &str,
    topic0: AA,
    topic1: BB,
    topic2: CC,
  ) -> crate::Result<Vector<R>>
  where
    AA: Tokenize,
    BB: Tokenize,
    CC: Tokenize,
    R: Detokenize,
    for<'filter> JsonRpcRequest<EthGetLogsReq<'filter>>: Encode<De<DRSR>>,
    JsonRpcResponse<Option<Vector<Log>>>: for<'de> DecodeSeq<'de, De<DRSR>>,
  {
    fn to_topic<A: Tokenize>(x: A) -> ethabi::Topic<ethabi::Token> {
      let tokens = x.into_tokens();
      if tokens.is_empty() { ethabi::Topic::Any } else { alloc::vec::Vec::from(tokens).into() }
    }

    let ev = self.abi.event(event)?;

    let topic_filer = ev.filter(ethabi::RawTopicFilter {
      topic0: to_topic(topic0),
      topic1: to_topic(topic1),
      topic2: to_topic(topic2),
    })?;

    let filter = FilterBuilder::default().topic_filter(topic_filer)?.build();
    let (pkgs_aux, trans) = self.ethereum.parts_mut();
    let mut pkg = pkgs_aux.eth_get_logs().data(&filter).build();
    let Some(logs) = trans.send_pkg_recv_decode_contained(&mut pkg, pkgs_aux).await?.result? else {
      return Ok(Vector::new());
    };

    Ok(
      logs
        .into_iter()
        .map(move |l| {
          let log =
            ev.parse_log(ethabi::RawLog { topics: l.topics.into(), data: l.data.0.into() })?;
          let vector =
            log.params.into_iter().map(|x| x.value).collect::<Wrapper<Result<Vector<_>, _>>>().0?;
          R::from_tokens(vector)
        })
        .collect::<Result<Wrapper<Result<Vector<_>, _>>, _>>()?
        .0?,
    )
  }

  /// Call constant function
  #[inline]
  pub async fn query<R, FP>(
    &mut self,
    func: &str,
    func_params: FP,
    from: Option<Address>,
    options: Options,
    block_id: Option<&BlockId>,
  ) -> crate::Result<Option<R>>
  where
    FP: Tokenize,
    R: Detokenize,
    for<'any, 'de> EthCallPkg<JsonRpcRequest<EthCallReq<'any>>>: Package<
        Ethereum,
        DRSR,
        T::Inner,
        HttpParams,
        ExternalResponseContent<'de> = JsonRpcResponse<Option<Bytes>>,
      >,
  {
    let function = self.abi.function(func)?;
    let bytes = function.encode_input(&func_params.into_tokens())?.into();
    let call_request = CallRequest {
      from: from.into(),
      to: Some(self.address),
      gas: options.gas,
      gas_price: options.gas_price,
      value: options.value,
      data: Some(Bytes(bytes)),
      ty: options.ty,
      access_list: options.access_list,
      max_fee_per_gas: options.max_fee_per_gas,
      max_priority_fee_per_gas: options.max_priority_fee_per_gas,
    };
    let (pkgs_aux, trans) = self.ethereum.parts_mut();
    let mut pkg = pkgs_aux.eth_call().data(block_id, &call_request).build();
    trans
      .send_pkg_recv_decode_contained(&mut pkg, pkgs_aux)
      .await?
      .result?
      .map(|el| R::from_tokens(function.decode_output(&el.0)?.into()))
      .transpose()
  }
}

#[cfg(test)]
mod tests {
  use crate::blockchain::ethereum::{
    BlockId, BlockNumber, CallRequest, Ethereum, EthereumPkgsAux,
    contract::{Contract, Detokenize, Options},
  };
  use alloc::{
    borrow::{Cow, ToOwned},
    format,
    string::String,
  };
  use ethabi::{Address, Token};
  use ethereum_types::{H256, U256};
  use serde::Serialize;
  use wtx::{
    client_api_framework::{
      misc::Pair,
      network::{HttpParams, transport::Mock},
    },
    collection::Vector,
    data_transformation::{
      dnsn::SerdeJson,
      format::{JsonRpcRequest, JsonRpcResponse},
    },
  };

  const HELLO_WORLD: &str = "0x00000000000000000000000000000000000000000000000000000000000000200000\
  00000000000000000000000000000000000000000000000000000000000c48656c6c6f20576f726c6421000000000000\
  0000000000000000000000000000";

  #[test]
  fn decoding_array_of_fixed_bytes() {
    let tokens = wtx::vector![Token::FixedArray(
      wtx::vector![
        Token::FixedBytes(hex::decode("01").unwrap().into()),
        Token::FixedBytes(hex::decode("02").unwrap().into()),
        Token::FixedBytes(hex::decode("03").unwrap().into()),
        Token::FixedBytes(hex::decode("04").unwrap().into()),
        Token::FixedBytes(hex::decode("05").unwrap().into()),
        Token::FixedBytes(hex::decode("06").unwrap().into()),
        Token::FixedBytes(hex::decode("07").unwrap().into()),
        Token::FixedBytes(hex::decode("08").unwrap().into()),
      ]
      .into()
    )];
    let data: [[u8; 1]; 8] = Detokenize::from_tokens(tokens).unwrap();
    assert_eq!(data[0][0], 1);
    assert_eq!(data[1][0], 2);
    assert_eq!(data[2][0], 3);
    assert_eq!(data[7][0], 8);
  }

  #[ignore]
  #[test]
  fn decoding_compiles() {
    let _address: Address = output();
    let _bool: bool = output();
    let _bytes: Vector<u8> = output();
    let _string: String = output();
    let _tokens: Vector<Token> = output();
    let _uint: U256 = output();

    let _array: [U256; 4] = output();
    let _bytes: Vector<[[u8; 1]; 64]> = output();
    let _pair: (U256, bool) = output();
    let _vec: Vector<U256> = output();

    let _mixed: (Vector<Vector<u8>>, [U256; 4], Vector<U256>, U256) = output();

    let _uints: (u16, u32, u64, u128) = output();
  }

  #[tokio::test]
  async fn should_call_constant_function() {
    let block_id = BlockId::Number(BlockNumber::Number(1));
    let mut trans = Mock::default();
    trans.push_response(response(HELLO_WORLD.into()));
    let result: String = contract(&mut trans)
      .query("name", (), None, Options::default(), Some(&block_id))
      .await
      .unwrap()
      .unwrap();
    assert_eq!(result, "Hello World!");
    let mut cr = call_request();
    cr.data = Some(hex::decode("06fdde03").unwrap().into());
    trans.assert_request(&req("eth_call", (block_id, cr)));
    trans.assert_does_not_have_non_asserted_requests();
  }

  #[tokio::test]
  async fn should_call_constant_function_by_hash() {
    let block_id = BlockId::Hash(H256::default());
    let mut trans = Mock::default();
    trans.push_response(response(HELLO_WORLD.into()));
    let result: String = contract(&mut trans)
      .query("name", (), None, Options::default(), Some(&block_id))
      .await
      .unwrap()
      .unwrap();
    assert_eq!(result, "Hello World!".to_owned());
    let mut cr = call_request();
    cr.data = Some(hex::decode("06fdde03").unwrap().into());
    trans.assert_request(&req("eth_call", (block_id, cr)));
    trans.assert_does_not_have_non_asserted_requests();
  }

  #[tokio::test]
  async fn should_query_with_params() {
    let block_id = BlockId::Number(BlockNumber::Latest);
    let from = Address::from_low_u64_be(5);
    let mut trans = Mock::default();
    trans.push_response(response(HELLO_WORLD.into()));
    let result: String = contract(&mut trans)
      .query(
        "name",
        (),
        Some(from),
        Options::with(|options| options.gas_price = Some(10_000_000.into())),
        Some(&block_id),
      )
      .await
      .unwrap()
      .unwrap();
    assert_eq!(result, "Hello World!".to_owned());
    let mut cr = call_request();
    cr.data = Some(hex::decode("06fdde03").unwrap().into());
    cr.from = Some(from);
    cr.gas_price = Some(10_000_000.into());
    trans.assert_request(&req("eth_call", (block_id, cr)));
    trans.assert_does_not_have_non_asserted_requests();
  }

  #[tokio::test]
  async fn should_call_a_contract_function() {
    let from = Address::from_low_u64_be(5);
    let mut trans = Mock::default();
    trans.push_response(response(format!("{:#x}", H256::from_low_u64_be(5)).into()));
    assert_eq!(
      contract(&mut trans).call("name", (), from, Options::default()).await.unwrap().unwrap(),
      H256::from_low_u64_be(5)
    );
    let mut cr = call_request();
    cr.data = Some(hex::decode("06fdde03").unwrap().into());
    cr.from = Some(from);
    trans.assert_request(&req("eth_sendTransaction", [cr]));
    trans.assert_does_not_have_non_asserted_requests();
  }

  #[tokio::test]
  async fn should_estimate_gas_usage() {
    let from = Address::from_low_u64_be(5);
    let mut trans = Mock::default();
    trans.push_response(response(format!("{:#x}", U256::from(5)).into()));
    assert_eq!(
      contract(&mut trans).estimate_gas("name", (), from, Options::default()).await.unwrap(),
      5.into()
    );
    let mut cr = call_request();
    cr.data = Some(hex::decode("06fdde03").unwrap().into());
    cr.from = Some(from);
    trans.assert_request(&req("eth_estimateGas", [cr]));
    trans.assert_does_not_have_non_asserted_requests();
  }

  #[tokio::test]
  async fn should_query_single_parameter_function() {
    let mut trans = Mock::default();
    trans.push_response(response(
      "0x0000000000000000000000000000000000000000000000000000000000000020".into(),
    ));
    let result: U256 = contract(&mut trans)
      .query(
        "balanceOf",
        Address::from_low_u64_be(5),
        None,
        Options::default(),
        Some(&BlockId::Number(BlockNumber::Latest)),
      )
      .await
      .unwrap()
      .unwrap();
    assert_eq!(result, 0x20.into());
    let mut cr = call_request();
    cr.data = Some(
      hex::decode("70a082310000000000000000000000000000000000000000000000000000000000000005")
        .unwrap()
        .into(),
    );
    trans.assert_request(&req("eth_call", (BlockId::Number(BlockNumber::Latest), cr)));
    trans.assert_does_not_have_non_asserted_requests();
  }

  fn call_request() -> CallRequest {
    let mut cr = CallRequest::default();
    cr.to = Some(Address::from_low_u64_be(1));
    cr
  }

  fn contract(
    trans: &mut Mock<str, HttpParams>,
  ) -> Contract<SerdeJson, &mut Mock<str, HttpParams>> {
    let pair = Pair::new(
      EthereumPkgsAux::from_minimum(
        Ethereum::new(None),
        SerdeJson,
        HttpParams::from_uri(String::new()),
      ),
      trans,
    );
    Contract::from_json(Address::from_low_u64_be(1), pair, include_bytes!("./resources/token.json"))
      .unwrap()
  }

  fn output<R>() -> R
  where
    R: Detokenize,
  {
    unimplemented!()
  }

  fn req<P>(method: &'static str, params: P) -> String
  where
    P: Serialize,
  {
    serde_json::to_string(&JsonRpcRequest { id: 1, method, params }).unwrap()
  }

  fn response(result: Cow<'static, str>) -> Cow<'static, str> {
    let elem = JsonRpcResponse { id: 1, method: None, result: Ok(result) };
    serde_json::to_string(&elem).unwrap().into()
  }
}
