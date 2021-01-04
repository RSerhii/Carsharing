use crate::state::PollStatus;
use cosmwasm_std::{HumanAddr, Uint128};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct InitMsg {
    pub title: String,
    pub kyc_verificator: HumanAddr,
    pub manager: HumanAddr
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum HandleMsg {
    RegisterCar {
        id: HumanAddr,
        name: String,
        rent_price: Uint128,
        deposit_price: Uint128
    },
    RegisterClient {
        name: String
    },
    VerifyClient {
        address: HumanAddr,
    }
    RentCar {
        car_id: HumanAddr,
        start: u64,
        end: u64
    },
    StartRent {
        rent_id: u64,
        date: u64,
    },
    EndRent {
        rent_id: u64,
        date: u64,
    },
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum QueryMsg {
    Balance { address: HumanAddr },
    Rent { rent_id: u64 },
}


#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct ClientBalanceResponse {
    pub balance: Uint128,
    pub locked_balance: Uint128
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct RentResponse {
    pub client: CanonicalAddr,
    pub car: CanonicalAddr,
    pub balance: Uint128,
    pub usage_start: u64
    pub usage_end: u64
    pub actual_start: u64
}

#[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema)]
pub struct RentCarResponse {
    pub rent_id: u64,
}