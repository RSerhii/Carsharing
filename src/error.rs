use cosmwasm_std::{StdError};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ContractError {
    #[error("{0}")]
    Std(#[from] StdError),

    #[error("Unauthorized")]
    Unauthorized {},

    #[error("Client does not exist")]
    ClientNotExist {},

    #[error("Car does not exist")]
    CarNotExist {},

    #[error("Rent does not exist")]
    RentNotExist {},

    #[error("Car already registered")]
    CarExist {},

    #[error("Client already registered")]
    ClientExist {},

    #[error("Client is not verified")]
    ClientNotVerified {},

    #[error("Insufficient funds")]
    InsufficientFunds {},

    #[error("Rent is closed")]
    RentClosed {},
}
