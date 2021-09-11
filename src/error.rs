use thiserror::Error;

use solana_program::program_error::ProgramError;

#[derive(Error, Debug, Copy, Clone)]
pub enum EscrowError {
  /// Invalid Instruction
  #[error("Invalid Instruction")]
  InvalidInstruction,
  /// Not Rent Exempt
  #[error("Escrow Account Not Rent Exempt")]
  NotRentExempt,
  /// Amount Expected in `Exchange` instruction doesn't match the amount in Escrow
  #[error("Taker expects different amount than Initializer")]
  ExpectedAmountMismatch,
  /// Overflow occured while moving lamports
  #[error("Overflow occured while moving lamports")]
  AmountOverflow,
}

impl From<EscrowError> for ProgramError {
  fn from(e: EscrowError) -> Self {
    ProgramError::Custom(e as u32)
  }
}
