// This file is auto-generated from the CIDL source.
// Editing this file directly is not recommended as it may be overwritten.

use num_derive::FromPrimitive;
use solana_program::decode_error::DecodeError;
use solana_program::msg;
use solana_program::program_error::{PrintProgramError, ProgramError};
use thiserror::Error;

#[derive(Error, FromPrimitive, Debug, Clone)]
pub enum MyDigitalArtNftError {
    #[error("Invalid Instruction")]
    InvalidInstruction,

    #[error("Invalid Signer Permission")]
    InvalidSignerPermission,

    #[error("Not The Expected Account Address")]
    NotExpectedAddress,

    #[error("Wrong Account Owner")]
    WrongAccountOwner,

    #[error("Invalid Account Len")]
    InvalidAccountLen,

    #[error("Executable Account Expected")]
    ExecutableAccountExpected,

 
}

impl From<MyDigitalArtNftError> for ProgramError {
    fn from(e: MyDigitalArtNftError) -> Self {
        ProgramError::Custom(e as u32)
    }
}

impl<T> DecodeError<T> for MyDigitalArtNftError {
    fn type_of() -> &'static str {
        "MyDigitalArtNftError"
    }
}

impl PrintProgramError for MyDigitalArtNftError {
    fn print<E>(&self)
    where
        E: 'static
            + std::error::Error
            + DecodeError<E>
            + PrintProgramError
            + num_traits::FromPrimitive,
    {
        match self {
            MyDigitalArtNftError::InvalidInstruction => msg!("Error: Invalid instruction"),
            MyDigitalArtNftError::InvalidSignerPermission => msg!("Error: The account is_signer value is not the expected one"),
            MyDigitalArtNftError::NotExpectedAddress => {
                msg!("Error: Not the expected account address")
            }
            MyDigitalArtNftError::WrongAccountOwner => msg!("Error: Wrong account owner"),
            MyDigitalArtNftError::InvalidAccountLen => msg!("Error: Invalid account length"),
            MyDigitalArtNftError::ExecutableAccountExpected => msg!("Error: Executable account expected"),
 
        }
    }
}