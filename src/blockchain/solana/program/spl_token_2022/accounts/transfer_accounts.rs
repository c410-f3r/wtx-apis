use crate::blockchain::solana::{
  InstructionAccountInput,
  misc::next_iter_elem,
  program::{AccountAddress, MAX_SIGNERS, PackAccounts, TestingInstances},
};
use wtx::{
  collection::{ArrayVectorU8, Vector},
  misc::Usize,
};

create_account_struct! {
  #[derive(Debug, PartialEq)]
  pub struct TransferAccounts<A> {
    source: A,
    destination: A,
    authority: A,
    pub signers: ArrayVectorU8<A, MAX_SIGNERS>,
  }
}

impl<A> PackAccounts<A> for TransferAccounts<A>
where
  A: AccountAddress,
{
  fn len(&self) -> usize {
    let mut sum: usize = 3;
    sum = sum.wrapping_add(*Usize::from(self.signers.len()));
    sum
  }

  fn push_accounts(&self, vec: &mut Vector<InstructionAccountInput>) -> crate::Result<()>
  where
    A: AccountAddress,
  {
    let Self { source, destination, authority, signers } = self;
    vec.extend_from_copyable_slice(&[
      InstructionAccountInput::writable(source.to_account_address()?),
      InstructionAccountInput::writable(destination.to_account_address()?),
      if signers.is_empty() {
        InstructionAccountInput::sign(authority.to_account_address()?)
      } else {
        InstructionAccountInput::none(authority.to_account_address()?)
      },
    ])?;
    for signer in signers {
      vec.push(InstructionAccountInput::sign(signer.to_account_address()?))?;
    }
    Ok(())
  }

  fn unpack_accounts(iter: &mut impl Iterator<Item = A>) -> crate::Result<Self> {
    let rslt = Self {
      source: next_iter_elem(iter)?,
      destination: next_iter_elem(iter)?,
      authority: next_iter_elem(iter)?,
      signers: ArrayVectorU8::from_iterator(iter)?,
    };
    Ok(rslt)
  }
}

impl TestingInstances for TransferAccounts<[u8; 32]> {
  fn min_instance() -> crate::Result<Self> {
    Ok(Self {
      source: [1; 32],
      destination: [1; 32],
      authority: [1; 32],
      signers: ArrayVectorU8::new(),
    })
  }
}
