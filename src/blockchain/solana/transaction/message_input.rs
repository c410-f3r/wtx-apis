use crate::blockchain::solana::{AddressLookupTableAccount, SolanaAddressHash, SolanaBlockhash};
use alloc::collections::BTreeMap;
use wtx::{collection::Vector, misc::Wrapper};

/// Compiled [InstructionInput]
#[derive(Debug, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CompiledInstructionInput {
  /// Index in regards to the block array of programs.
  pub program_id_index: u8,
  /// Indexes in regards to the block array of accounts.
  #[serde(with = "crate::blockchain::solana::short_vec")]
  pub accounts: Vector<u8>,
  /// Opaque bytes
  #[serde(with = "crate::blockchain::solana::short_vec")]
  pub data: Vector<u8>,
}

/// Used when performing requests
//
// The order of the fields is important
#[derive(Debug, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct InstructionInput {
  /// Base58 identifier
  pub program_id: SolanaAddressHash,
  /// List of necessary accounts
  pub accounts: Vector<InstructionAccountInput>,
  /// Opaque data
  pub data: Vector<u8>,
}

/// Account information.
#[derive(Clone, Copy, Debug, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct InstructionAccountInput {
  /// Base58 identifier.
  pub pubkey: SolanaAddressHash,
  /// Signed the transaction.
  pub is_signer: bool,
  /// Had state modified.
  pub is_writable: bool,
}

impl InstructionAccountInput {
  /// Account is not a signer nor writable.
  pub fn none(pubkey: SolanaAddressHash) -> Self {
    Self { pubkey, is_signer: false, is_writable: false }
  }

  /// Account is signer but not writable.
  pub fn sign(pubkey: SolanaAddressHash) -> Self {
    Self { pubkey, is_signer: true, is_writable: false }
  }

  /// Account is signer and writable
  pub fn sign_and_writable(pubkey: SolanaAddressHash) -> Self {
    Self { pubkey, is_signer: true, is_writable: true }
  }

  /// Account is writable but not signer.
  pub fn writable(pubkey: SolanaAddressHash) -> Self {
    Self { pubkey, is_signer: false, is_writable: true }
  }
}

/// On-chain address lookup that is used in a single tx.
#[derive(Debug, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct MessageAddressTableLookup {
  /// Identifier
  pub account_key: SolanaBlockhash,
  /// Writeable account indexes
  #[serde(with = "crate::blockchain::solana::short_vec")]
  pub writable_indexes: Vector<u8>,
  /// Readonly account indexes
  #[serde(with = "crate::blockchain::solana::short_vec")]
  pub readonly_indexes: Vector<u8>,
}

/// Internal auxiliary structure used to create a message
#[derive(Debug, Default)]
pub struct MessageBuffer {
  adhocs: Vector<(SolanaAddressHash, MessageBufferUniqueElem)>,
  all: BTreeMap<SolanaAddressHash, MessageBufferUniqueElem>,
  alta_readonly_pubkeys: Vector<SolanaAddressHash>,
  alta_writable_pubkeys: Vector<SolanaAddressHash>,
}

impl MessageBuffer {
  fn clear(&mut self) {
    let Self { adhocs, all, alta_readonly_pubkeys, alta_writable_pubkeys } = self;
    adhocs.clear();
    all.clear();
    alta_readonly_pubkeys.clear();
    alta_writable_pubkeys.clear();
  }
}

/// Message
#[derive(Debug, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct MessageInput {
  /// Header
  pub header: MessageHeaderInput,
  /// All block accounts
  #[serde(with = "crate::blockchain::solana::short_vec")]
  pub account_keys: Vector<SolanaAddressHash>,
  /// Recent blockhash
  pub recent_blockhash: SolanaBlockhash,
  /// All block instructions
  #[serde(with = "crate::blockchain::solana::short_vec")]
  pub instructions: Vector<CompiledInstructionInput>,
  /// Address table lookups
  #[serde(with = "crate::blockchain::solana::short_vec")]
  pub address_table_lookups: Vector<MessageAddressTableLookup>,
}

impl MessageInput {
  /// Takes all the necessary parameters to validate and transform data into a suitable format for
  /// submission.
  pub fn with_params(
    altas: &[AddressLookupTableAccount],
    buffer: &mut MessageBuffer,
    instructions: &[InstructionInput],
    payer: Option<SolanaAddressHash>,
    recent_blockhash: SolanaBlockhash,
  ) -> crate::Result<Self> {
    buffer.clear();
    let mut fun = || {
      let mut address_table_lookups = Vector::with_capacity(altas.len())?;
      Self::fill_buffer_with_unique_accounts(buffer, instructions, payer);
      for alta in altas {
        Self::extract_table_lookup_accounts(&mut address_table_lookups, alta, buffer)?;
      }
      let (header, account_keys) = Self::build_adhoc_params(buffer, payer)?;
      let mut compiled_instructions = Vector::new();
      for instruction in instructions {
        compiled_instructions.push(Self::compile_instruction(
          &account_keys,
          buffer,
          instruction,
        )?)?;
      }
      Ok(Self {
        account_keys,
        address_table_lookups,
        header,
        instructions: compiled_instructions,
        recent_blockhash,
      })
    };
    match fun() {
      Err(err) => {
        buffer.clear();
        Err(err)
      }
      Ok(elem) => Ok(elem),
    }
  }

  fn compile_instruction(
    adhoc_pubkeys: &[SolanaAddressHash],
    buffer: &mut MessageBuffer,
    instruction: &InstructionInput,
  ) -> crate::Result<CompiledInstructionInput> {
    if buffer
      .alta_readonly_pubkeys
      .len()
      .checked_add(buffer.alta_writable_pubkeys.len())
      .and_then(|el| u8::try_from(el).ok())
      .is_none()
    {
      return Err(crate::Error::SolanaU8Overflow);
    }
    let all = [adhoc_pubkeys, &buffer.alta_writable_pubkeys, &buffer.alta_readonly_pubkeys];
    let mut idx: u8 = 0;
    buffer.all.clear();
    for pubkey in all.into_iter().flatten().copied() {
      let _ = buffer.all.insert(
        pubkey,
        MessageBufferUniqueElem { idx, is_invoked: false, is_signer: false, is_writable: false },
      );
      idx = idx.wrapping_add(1);
    }
    let position = |pubkey: &[u8; 32]| {
      buffer.all.get(pubkey).map(|el| el.idx).ok_or(crate::Error::SolanaUnknownIxPubKey)
    };
    Ok(CompiledInstructionInput {
      program_id_index: position(&instruction.program_id)?,
      data: instruction.data.clone(),
      accounts: instruction
        .accounts
        .iter()
        .map(|elem| position(&elem.pubkey))
        .collect::<Result<Wrapper<Result<Vector<_>, _>>, _>>()?
        .0?,
    })
  }

  fn build_adhoc_params(
    buffer: &mut MessageBuffer,
    payer: Option<SolanaAddressHash>,
  ) -> crate::Result<(MessageHeaderInput, Vector<SolanaAddressHash>)> {
    let mut num_of_writable_signers: usize = if let Some(elem) = payer.as_ref() {
      let _ = buffer.all.remove(elem);
      1
    } else {
      0
    };
    let mut num_of_readonly_signers: usize = 0;
    let mut num_of_writable_non_signers: usize = 0;
    let mut num_of_readonly_non_signers: usize = 0;

    buffer.adhocs.extend_from_iter(buffer.all.iter().map(|el| (*el.0, *el.1)))?;
    buffer.adhocs.sort_by(|first, second| {
      second
        .1
        .is_signer
        .cmp(&first.1.is_signer)
        .then(second.1.is_writable.cmp(&first.1.is_writable))
    });

    for (_, elem) in buffer.adhocs.iter() {
      match [elem.is_signer, elem.is_writable] {
        [true, true] => num_of_writable_signers = num_of_writable_signers.wrapping_add(1),
        [true, false] => num_of_readonly_signers = num_of_readonly_signers.wrapping_add(1),
        [false, true] => num_of_writable_non_signers = num_of_writable_non_signers.wrapping_add(1),
        [false, false] => num_of_readonly_non_signers = num_of_readonly_non_signers.wrapping_add(1),
      }
    }

    let convert = |n: usize| u8::try_from(n).map_err(|_| crate::Error::SolanaU8Overflow);
    Ok((
      MessageHeaderInput {
        num_readonly_signed_accounts: convert(num_of_readonly_signers)?,
        num_readonly_unsigned_accounts: convert(num_of_readonly_non_signers)?,
        num_required_signatures: convert(
          num_of_writable_signers.saturating_add(num_of_readonly_signers),
        )?,
      },
      payer
        .into_iter()
        .chain(buffer.adhocs.iter().map(|el| el.0))
        .collect::<Wrapper<Result<Vector<_>, _>>>()
        .0?,
    ))
  }

  fn extract_table_lookup_accounts(
    address_table_lookups: &mut Vector<MessageAddressTableLookup>,
    alta: &AddressLookupTableAccount,
    buffer: &mut MessageBuffer,
  ) -> crate::Result<()> {
    if alta.addresses.len() > 256 {
      return Err(crate::Error::SolanaU8Overflow);
    }
    let mut readonly_indexes = Vector::new();
    let mut writable_indexes = Vector::new();
    #[expect(clippy::unwrap_used, reason = "`retain` does not allow the use of results")]
    buffer.all.retain(|pubkey, elem| {
      let mut alta_idx: u8 = 0;
      for alta_pubkey in alta.addresses.iter() {
        if pubkey != alta_pubkey {
          alta_idx = alta_idx.wrapping_add(1);
          continue;
        }
        match [elem.is_invoked, elem.is_signer, elem.is_writable] {
          [false, false, false] => {
            buffer.alta_readonly_pubkeys.push(*pubkey).unwrap();
            readonly_indexes.push(alta_idx).unwrap();
            return false;
          }
          [false, false, true] => {
            buffer.alta_writable_pubkeys.push(*pubkey).unwrap();
            writable_indexes.push(alta_idx).unwrap();
            return false;
          }
          _ => {}
        }
        alta_idx = alta_idx.wrapping_add(1);
      }
      true
    });
    if writable_indexes.is_empty() && readonly_indexes.is_empty() {
      return Ok(());
    }
    address_table_lookups.push(MessageAddressTableLookup {
      account_key: alta.key,
      writable_indexes,
      readonly_indexes,
    })?;
    Ok(())
  }

  fn fill_buffer_with_unique_accounts(
    buffer: &mut MessageBuffer,
    instructions: &[InstructionInput],
    payer: Option<SolanaAddressHash>,
  ) {
    for instruction in instructions.iter() {
      let _ = buffer.all.insert(
        instruction.program_id,
        MessageBufferUniqueElem { idx: 0, is_invoked: true, is_signer: false, is_writable: false },
      );
      for instruction_account in instruction.accounts.iter() {
        let _ = buffer.all.insert(
          instruction_account.pubkey,
          MessageBufferUniqueElem {
            idx: 0,
            is_invoked: false,
            is_signer: instruction_account.is_signer,
            is_writable: instruction_account.is_writable,
          },
        );
      }
    }
    if let Some(pubkey) = payer {
      let _ = buffer.all.insert(
        pubkey,
        MessageBufferUniqueElem { idx: 0, is_invoked: false, is_signer: true, is_writable: true },
      );
    }
  }
}

/// Header containing overall account information.
#[derive(Debug, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct MessageHeaderInput {
  /// Number of required signatures
  pub num_required_signatures: u8,
  /// Number of readonly signed accounts
  pub num_readonly_signed_accounts: u8,
  /// Number of readonly unsigned accounts.
  pub num_readonly_unsigned_accounts: u8,
}

#[derive(Clone, Copy, Debug)]
struct MessageBufferUniqueElem {
  is_signer: bool,
  is_writable: bool,
  idx: u8,
  is_invoked: bool,
}

#[cfg(test)]
mod tests {
  use wtx::collection::Vector;

  use crate::blockchain::solana::{
    CompiledInstructionInput, InstructionAccountInput, InstructionInput, MessageAddressTableLookup,
    MessageHeaderInput, MessageInput, address_lookup_table_account::AddressLookupTableAccount,
  };

  #[test]
  fn with_params_generates_correct_output() {
    let mut buffer = <_>::default();
    let mut call = || {
      let pubkeys =
        [[0; 32], [1; 32], [2; 32], [3; 32], [4; 32], [5; 32], [6; 32], [7; 32], [8; 32], [9; 32]];
      let payer = pubkeys[0];
      let program_id = pubkeys[6];
      let instructions = wtx::vector![InstructionInput {
        program_id,
        accounts: wtx::vector![
          InstructionAccountInput::sign_and_writable(pubkeys[1]),
          InstructionAccountInput::sign(pubkeys[2]),
          InstructionAccountInput::writable(pubkeys[3]),
          InstructionAccountInput::writable(pubkeys[4]),
          InstructionAccountInput::none(pubkeys[5]),
        ],
        data: Vector::new(),
      }];
      let address_lookup_table_accounts = wtx::vector![
        AddressLookupTableAccount {
          key: pubkeys[7],
          addresses: wtx::vector![pubkeys[4], pubkeys[5], pubkeys[6]],
        },
        AddressLookupTableAccount { key: pubkeys[8], addresses: Vector::new() },
      ];

      let recent_blockhash = pubkeys[9];
      assert_eq!(
        MessageInput::with_params(
          &address_lookup_table_accounts,
          &mut buffer,
          &instructions,
          Some(payer),
          recent_blockhash
        )
        .unwrap(),
        MessageInput {
          account_keys: wtx::vector![pubkeys[0], pubkeys[1], pubkeys[2], pubkeys[3], program_id],
          address_table_lookups: wtx::vector![MessageAddressTableLookup {
            account_key: address_lookup_table_accounts[0].key,
            writable_indexes: wtx::vector![0],
            readonly_indexes: wtx::vector![1],
          }],
          header: MessageHeaderInput {
            num_required_signatures: 3,
            num_readonly_signed_accounts: 1,
            num_readonly_unsigned_accounts: 1
          },
          instructions: wtx::vector![CompiledInstructionInput {
            program_id_index: 4,
            accounts: wtx::vector![1, 2, 3, 5, 6],
            data: wtx::vector![],
          }],
          recent_blockhash,
        }
      );
    };
    call();
    call();
  }
}
