#[wtx::pkg(
  data_format(json_rpc("sendTransaction")),
  id(crate::blockchain::solana::SolanaId),
  transport(http)
)]
pub(crate) mod pkg {
  use crate::blockchain::solana::{
    Commitment, HttpPkgsAux, SendTransactionEncoding, SolanaTransactionHashStr, TransactionInput,
  };
  use wtx::{
    client_api_framework::network::transport::TransportParams,
    codec::{
      Base64Alphabet, Encode, EncodeWrapper, GenericCodec, base64_encode, protocol::VerbatimEncoder,
    },
  };

  #[pkg::aux]
  impl<A, DRSR> HttpPkgsAux<A, DRSR> {
    #[pkg::aux_data]
    fn send_transaction_data(
      &mut self,
      config: Option<SendTransactionConfig>,
      tx: &TransactionInput,
    ) -> crate::Result<SendTransactionReq>
    where
      for<'any, 'drsr> VerbatimEncoder<SendTransactionReqInner<'any>>:
        Encode<GenericCodec<&'drsr mut DRSR, &'drsr mut DRSR>>,
    {
      let this = &mut **self;
      this.bytes_buffer.clear();
      bincode::serialize_into(&mut this.bytes_buffer, tx)?;
      let encoded = if let Some(SendTransactionConfig {
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
      VerbatimEncoder::new(SendTransactionReqInner(encoded, config))
        .encode(&mut EncodeWrapper::new(&mut this.bytes_buffer, &mut this.drsr))?;
      this.tp.ext_req_params_mut().rrb.body.clear();
      this.encode_data = true;
      Ok(())
    }
  }

  #[pkg::req_data]
  pub type SendTransactionReq = ();

  #[derive(Debug, serde::Serialize)]
  #[doc = generic_config_doc!()]
  #[serde(rename_all = "camelCase")]
  pub struct SendTransactionConfig {
    /// Encoding
    #[serde(skip_serializing_if = "Option::is_none")]
    pub encoding: Option<SendTransactionEncoding>,
    /// Maximum number of times for the RPC node to retry sending the transaction to the leader. If
    /// this parameter not provided, the RPC node will retry the transaction until it is finalized
    /// or until the blockhash expires.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_retries: Option<usize>,
    #[doc = min_context_slot_doc!()]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_context_slot: Option<u64>,
    #[doc = commitment_doc!()]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub preflight_commitment: Option<Commitment>,
    /// If true, skip the preflight transaction checks
    pub skip_preflight: bool,
  }

  #[pkg::res_data]
  pub type SendTransactionRes = SolanaTransactionHashStr;

  #[derive(Debug, serde::Serialize)]
  struct SendTransactionReqInner<'any>(
    /// Encoded tx
    pub &'any [u8],
    /// Configuration
    #[serde(skip_serializing_if = "Option::is_none")]
    pub Option<SendTransactionConfig>,
  );
}
