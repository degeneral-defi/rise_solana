use borsh::{BorshDeserialize, BorshSerialize};
// use borsh_derive::{BorshDeserialize, BorshSerialize};
use solana_program::program_error::ProgramError;

#[derive(Debug, BorshSerialize, BorshDeserialize)]
pub struct IncrementArgs {
    pub value: u32
}
#[derive(Debug, BorshSerialize, BorshDeserialize)]
pub struct UpdateArgs {
    pub value: u32
}
pub enum CounterInstruction {
    Increment(IncrementArgs),
    Decrement(IncrementArgs),
    Update(UpdateArgs),
    Reset,
}

impl CounterInstruction {
    pub fn unpack(input: &[u8]) -> Result<Self, ProgramError> {
        let (&variant, rest) = input
            .split_first()
            .ok_or(ProgramError::InvalidInstructionData)?;

        Ok(match variant {
            0 => Self::Increment(IncrementArgs::try_from_slice(rest).unwrap()),
            1 => Self::Decrement(IncrementArgs::try_from_slice(rest).unwrap()),
            2 => Self::Update(UpdateArgs::try_from_slice(rest).unwrap()),
            3 => Self::Reset,
            _ => return Err(ProgramError::InvalidInstructionData),
        })
    }
}
