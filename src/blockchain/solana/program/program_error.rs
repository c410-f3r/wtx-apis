/// Program error
#[derive(Clone, Copy, Debug)]
pub enum ProgramError {
  /// Custom program error
  Custom(u32),
  /// The arguments provided to a program instruction were invalid
  InvalidArgument,
  /// An instruction's data contents was invalid
  InvalidInstructionData,
  /// An account's data contents was invalid
  InvalidAccountData,
  /// An account's data was too small
  AccountDataTooSmall,
  /// An account's balance was too small to complete the instruction
  InsufficientFunds,
  /// The account did not have the expected program id
  IncorrectProgramId,
  /// A signature was required but not found
  MissingRequiredSignature,
  /// An initialize instruction was sent to an account that has already been initialized
  AccountAlreadyInitialized,
  /// An attempt to operate on an account that hasn't been initialized
  UninitializedAccount,
  /// The instruction expected additional account keys
  NotEnoughAccountKeys,
  /// Failed to borrow a reference to account data, already borrowed
  AccountBorrowFailed,
  /// Length of the seed is too long for address generation
  MaxSeedLengthExceeded,
  /// Provided seeds do not result in a valid address
  InvalidSeeds,
  /// IO Error
  BorshIoError,
  /// An account does not have enough lamports to be rent-exempt
  AccountNotRentExempt,
  /// Unsupported sysvar
  UnsupportedSysvar,
  /// Provided owner is not allowed
  IllegalOwner,
  /// Accounts data allocations exceeded the maximum allowed per transaction
  MaxAccountsDataAllocationsExceeded,
  /// Account data reallocation was invalid
  InvalidRealloc,
  /// Instruction trace length exceeded the maximum allowed per transaction
  MaxInstructionTraceLengthExceeded,
  /// Builtin programs must consume compute units
  BuiltinProgramsMustConsumeComputeUnits,
  /// Invalid account owner
  InvalidAccountOwner,
  /// Program arithmetic overflowed
  ArithmeticOverflow,
  /// Account is immutable
  Immutable,
  /// Incorrect authority provided
  IncorrectAuthority,
}

impl From<ProgramError> for u64 {
  #[inline]
  fn from(from: ProgramError) -> Self {
    match from {
      ProgramError::Custom(_) => 0,
      ProgramError::InvalidArgument => 1,
      ProgramError::InvalidInstructionData => 2,
      ProgramError::InvalidAccountData => 3,
      ProgramError::AccountDataTooSmall => 4,
      ProgramError::InsufficientFunds => 5,
      ProgramError::IncorrectProgramId => 6,
      ProgramError::MissingRequiredSignature => 7,
      ProgramError::AccountAlreadyInitialized => 8,
      ProgramError::UninitializedAccount => 9,
      ProgramError::NotEnoughAccountKeys => 10,
      ProgramError::AccountBorrowFailed => 11,
      ProgramError::MaxSeedLengthExceeded => 12,
      ProgramError::InvalidSeeds => 13,
      ProgramError::BorshIoError => 14,
      ProgramError::AccountNotRentExempt => 15,
      ProgramError::UnsupportedSysvar => 16,
      ProgramError::IllegalOwner => 17,
      ProgramError::MaxAccountsDataAllocationsExceeded => 18,
      ProgramError::InvalidRealloc => 19,
      ProgramError::MaxInstructionTraceLengthExceeded => 20,
      ProgramError::BuiltinProgramsMustConsumeComputeUnits => 21,
      ProgramError::InvalidAccountOwner => 22,
      ProgramError::ArithmeticOverflow => 23,
      ProgramError::Immutable => 24,
      ProgramError::IncorrectAuthority => 25,
    }
  }
}
