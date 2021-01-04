use cosmwasm_std::{CanonicalAddr, StdError, Uint128};
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
}
