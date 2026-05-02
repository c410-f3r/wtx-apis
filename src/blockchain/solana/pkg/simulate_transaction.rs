#[wtx::pkg(
  data_format(json_rpc("simulateTransaction")),
  id(crate::blockchain::solana::SolanaId),
  transport(http)
)]
pub(crate) mod pkg {
  use crate::blockchain::solana::{
    AccountEncoding, Commitment, HttpPkgsAux, SendTransactionEncoding, TransactionInput,
  };
  use serde::Serialize;
  use wtx::{
    client_api_framework::network::transport::TransportParams,
    codec::{
      Base64Alphabet, Encode, EncodeWrapper, GenericCodec, base64_encode, protocol::VerbatimEncoder,
    },
  };

  #[pkg::aux]
  impl<A, DRSR> HttpPkgsAux<A, DRSR> {
    #[pkg::aux_data]
    fn simulate_transaction_data<ADDR>(
      &mut self,
      config: Option<SimulateTransactionConfig<ADDR>>,
      tx: &TransactionInput,
    ) -> crate::Result<()>
    where
      ADDR: Serialize,
      for<'any, 'drsr> VerbatimEncoder<SimulateTransactionReqInner<'any, ADDR>>:
        Encode<GenericCodec<&'drsr mut DRSR, &'drsr mut DRSR>>,
    {
      let this = &mut **self;
      this.bytes_buffer.clear();
      bincode::serialize_into(&mut this.bytes_buffer, tx)?;
      let encoded = if let Some(SimulateTransactionConfig {
        encoding: Some(SendTransactionEncoding::Base64),
        ..
      }) = config
      {
        base64_encode(
          Base64Alphabet::Standard,
          &this.bytes_buffer,
          &mut this.tp.ext_req_params_mut().rrb.body,
        )?
        .as_bytes()
      } else {
        let idx = bs58::encode(&this.bytes_buffer)
          .onto(&mut *this.tp.ext_req_params_mut().rrb.body)
          .map_err(|_err| crate::Error::Bs58Error)?;
        this.tp.ext_req_params_mut().rrb.body.get(..idx).unwrap_or_default()
      };
      this.bytes_buffer.clear();
      VerbatimEncoder::new(SimulateTransactionReqInner(encoded, config))
        .encode(&mut EncodeWrapper::new(&mut this.bytes_buffer, &mut this.drsr))?;
      this.tp.ext_req_params_mut().rrb.body.clear();
      this.encode_data = true;
      Ok(())
    }
  }

  #[pkg::req_data]
  pub type SimulateTransactionReq = ();

  #[pkg::res_data]
  pub type SimulateTransactionRes = ();

  #[derive(Debug, serde::Serialize)]
  #[doc = generic_config_doc!()]
  #[serde(rename_all = "camelCase")]
  pub struct SimulateTransactionConfig<ADDR> {
    /// If true the transaction signatures will be verified
    pub sig_verify: bool,
    #[doc = commitment_doc!()]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub commitment: Option<Commitment>,
    /// Encoding
    #[serde(skip_serializing_if = "Option::is_none")]
    pub encoding: Option<SendTransactionEncoding>,
    /// Accounts configuration
    #[serde(skip_serializing_if = "Option::is_none")]
    pub accounts: Option<SimulateTransactionAccounts<ADDR>>,
    /// If true, the transaction recent blockhash will be replaced with the most recent blockhash.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub replace_recent_blockhash: Option<bool>,
    #[doc = min_context_slot_doc!()]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_context_slot: Option<u64>,
  }

  /// Accounts configuration
  #[derive(Debug, serde::Serialize)]
  pub struct SimulateTransactionAccounts<ADDR> {
    addresses: ADDR,
    encoding: AccountEncoding,
  }

  #[derive(Debug, serde::Serialize)]
  struct SimulateTransactionReqInner<'any, ADDR>(
    &'any [u8],
    Option<SimulateTransactionConfig<ADDR>>,
  );
}
