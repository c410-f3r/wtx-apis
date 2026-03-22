use crate::blockchain::solana::{
  InstructionInput,
  program::{AccountAddress, PackAccounts, PackData},
};
use wtx::collection::Vector;

/// Trait for constructing Solana instruction inputs from packed accounts and data.
pub trait BuildIxInput<A>
where
  A: AccountAddress,
{
  /// The associated type responsible for packing instruction accounts.
  type PackAccounts: PackAccounts<A>;
  /// The associated type representing the raw instruction data.
  type PackData: PackData;
  /// The associated type representing the final, transformed instruction data.
  type TransformedPackData: PackData;

  /// Constructs an instruction input using the provided accounts, data, and program ID.
  fn build_ix_input(
    pack_accounts: Self::PackAccounts,
    pack_data: Self::PackData,
    program_id: A,
  ) -> crate::Result<InstructionInput> {
    let mut accounts = Vector::new();
    pack_accounts.push_accounts(&mut accounts)?;
    let mut data = Vector::new();
    Self::transform_pack_data(pack_data).pack_data(&mut data)?;
    Ok(InstructionInput { accounts, data, program_id: program_id.to_account_address()? })
  }

  /// Converts the input data into its final serializable representation.
  fn transform_pack_data(pack_data: Self::PackData) -> Self::TransformedPackData;
}
