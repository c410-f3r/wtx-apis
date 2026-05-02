#[wtx::pkg(
  data_format(json_rpc("getFeeForMessage")),
  id(crate::blockchain::solana::SolanaId),
  transport(http)
)]
pub(crate) mod pkg {
  use crate::blockchain::solana::{
    Commitment, HttpPkgsAux, JsonRpcResponseResultWithContext, MessageInput,
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
    fn get_fee_for_message_data(
      &mut self,
      config: Option<GetFeeForMessageConfig>,
      message: &MessageInput,
    ) -> crate::Result<GetFeeForMessageReq>
    where
      for<'any, 'drsr> VerbatimEncoder<GetFeeForMessageReqInner<'any>>:
        Encode<GenericCodec<&'drsr mut DRSR, &'drsr mut DRSR>>,
    {
      let this = &mut **self;
      this.bytes_buffer.clear();
      bincode::serialize_into(&mut this.bytes_buffer, message)?;
      let string = base64_encode(
        Base64Alphabet::Standard,
        &this.bytes_buffer,
        &mut this.tp.ext_req_params_mut().rrb.body,
      )?;
      this.bytes_buffer.clear();
      VerbatimEncoder::new(GetFeeForMessageReqInner(string, config))
        .encode(&mut EncodeWrapper::new(&mut this.bytes_buffer, &mut this.drsr))?;
      this.tp.ext_req_params_mut().rrb.body.clear();
      this.encode_data = true;
      Ok(())
    }
  }

  #[pkg::req_data]
  pub type GetFeeForMessageReq = ();

  #[pkg::res_data]
  pub type GetFeeForMessageRes = JsonRpcResponseResultWithContext<Option<u64>>;

  #[derive(Debug, serde::Serialize)]
  #[doc = generic_config_doc!()]
  #[serde(rename_all = "camelCase")]
  pub struct GetFeeForMessageConfig {
    #[doc = commitment_doc!()]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub commitment: Option<Commitment>,
    #[doc = min_context_slot_doc!()]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_context_slot: Option<u64>,
  }

  #[derive(Debug, serde::Serialize)]
  struct GetFeeForMessageReqInner<'any>(&'any str, Option<GetFeeForMessageConfig>);
}
