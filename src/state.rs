use cosmwasm_std::{CanonicalAddr, Storage};
use cosmwasm_storage::{
    bucket, bucket_read, singleton, singleton_read, Bucket, ReadonlyBucket, ReadonlySingleton,
    Singleton,
};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

static CONFIG_KEY: &[u8] = b"config";
static CARS_KEY: &[u8] = b"cars";
static CLIENTS_KEY: &[u8] = b"clients";
static RENTS_KEY: &[u8] = b"rents";

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct Config {
    pub denom: String,
    pub kyc_verificator: CanonicalAddr,
    pub manager: CanonicalAddr,
    pub rent_count: u64,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct TimePeriod {
    pub start: u64,
    pub end: u64
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct Car {
    pub id: CanonicalAddr,
    pub name: String,
    pub rent_price: u128,
    pub deposit_price: u128,
    pub usage_periods: Vec<TimePeriod>,
    pub balance: u128,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct Client {
    pub id: CanonicalAddr,
    pub name: String,
    pub verified: bool, 
    pub balance: u128,
    pub locked_balance: u128
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct Rent {
    pub client_id: CanonicalAddr,
    pub car_id: CanonicalAddr,
    pub balance: u128,
    pub usage: TimePeriod,
    pub actual_start: u64
}


pub fn config(storage: &mut dyn Storage) -> Singleton<Config> {
    singleton(storage, CONFIG_KEY)
}

pub fn config_read(storage: &dyn Storage) -> ReadonlySingleton<Config> {
    singleton_read(storage, CONFIG_KEY)
}

pub fn cars(storage: &mut dyn Storage) -> Bucket<Car> {
    bucket(storage, CARS_KEY)
}

pub fn cars_read(storage: &dyn Storage) -> ReadonlyBucket<Car> {
    bucket_read(storage, CARS_KEY)
}

pub fn clients(storage: &mut dyn Storage) -> Bucket<Client> {
    bucket(storage, CLIENTS_KEY)
}

pub fn clients_read(storage: &dyn Storage) -> ReadonlyBucket<Client> {
    bucket_read(storage, CLIENTS_KEY)
}

pub fn rents(storage: &mut dyn Storage) -> Bucket<Rent> {
    bucket(storage, RENTS_KEY)
}

pub fn rents_read(storage: &dyn Storage) -> ReadonlyBucket<Rent> {
    bucket_read(storage, RENTS_KEY)
}