use num_derive::FromPrimitive;
use solana_program::{
    decode_error::DecodeError,
    msg,
    program_error::{PrintProgramError, ProgramError},
};
use thiserror::Error;

#[derive(Clone, Debug, Eq, Error, PartialEq, FromPrimitive)]
pub enum ArcaderyError {
    #[error("You are not authorized to perform this action.")]
    Unauthorized = 0x71c7ac,
}

impl PrintProgramError for ArcaderyError {
    fn print<E>(&self) {
        msg!(&self.to_string());
    }
}

impl From<ArcaderyError> for ProgramError {
    fn from(e: ArcaderyError) -> Self {
        ProgramError::Custom(e as u32)
    }
}

impl<T> DecodeError<T> for ArcaderyError {
    fn type_of() -> &'static str {
        "Arcadery Error"
    }
}
