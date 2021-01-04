use crate::error::ContractError;
use crate::msg::{
    HandleMsg, InitMsg, QueryMsg, ClientBalanceResponse, RentResponse
};
use crate::state::{
    Config, TimePeriod, Car, Client, Rent, config, config_read, cars, cars_read, clients, clients_read, rents, rents_read
};
use cosmwasm_std::{
    attr, coin, to_binary, BankMsg, Binary, CanonicalAddr, Coin, CosmosMsg, Deps, DepsMut, Env,
    HandleResponse, HumanAddr, InitResponse, MessageInfo, StdError, StdResult, Storage, Uint128,
};

const RENT_PERIOD: u64 = 60;

pub fn init(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: InitMsg,
) -> Result<InitResponse, ContractError> {
    let config = Config {
        title: msg.title,
        kyc_verificator: deps.api.canonical_address(&msg.kyc_verificator),
        manager: deps.api.canonical_address(&msg.manager),
        rent_count: 0
    };

    config(deps.storage).save(&config)?;

    Ok(InitResponse::default())
}

pub fn handle(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: HandleMsg,
) -> Result<HandleResponse, ContractError> {
    match msg {
        HandleMsg::RegisterCar {
            id,
            name,
            rent_price,
            deposit_price
        } => register_car(deps, env, info, id, name, rent_price, deposit_price),
        HandleMsg::RegisterClient {
            name
        } => register_client(deps, env, info, name),
        VerifyClient {
            address: HumanAddr,
        } => verify_client(deps, env, info, address),
        HandleMsg::RentCar {
            car_id,
            start,
            end
        } => rent_car(deps, env, info, car_id, start, end),
        HandleMsg::StartRent {
            rent_id,
            date,
        } => start_rent(deps, env, info, rent_id, date),
        HandleMsg::EndRent {
            rent_id,
            date,
        } => end_rent(deps, env, info, rent_id, date),
    }
}

pub fn register_car(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    id: HumanAddr,
    name: String,
    rent_price: Uint128,
    deposit_price: Uint128
) -> Result<HandleResponse, ContractError> {
    let sender_address_raw = deps.api.canonical_address(&info.sender)?;
    let config_state = config(deps.storage).load()?;

    if sender_address_raw != config_state.manager {
        return Err(ContractError::Unauthorized {});
    }
    
    let car_address_raw = deps.api.canonical_address(&id)?;
    let key = &car_address_raw.as_slice();
    
    let stored_car = cars_read(deps.storage).may_load(key)?;
    if stored_car.is_some() {
        return Err(StdError::generic_err("Car already registered"));
    }
    
    let car = Car {
        id: car_address_raw,
        name: name,
        rent_price: rent_price, 
        deposit_price: deposit_price,
        balance: 0
    };

    cars(deps.storage).save(key, &car)?;

    Ok(HandleResponse::default())
}

pub fn register_client(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    name: String
) -> Result<HandleResponse, ContractError> {
    let sender_address_raw = deps.api.canonical_address(&info.sender)?;
    let key = &sender_address_raw.as_slice();
    
    let stored_client = clients_read(deps.storage).may_load(key)?;
    if stored_client.is_some() {
        return Err(StdError::generic_err("Client already registered"));
    }
    
    let client = Client {
        id: sender_address_raw,
        name: name,
        verified: False, 
        balance: &info.sent_funds,
        locked_balance: 0
    };

    clients(deps.storage).save(key, &client)?;

    Ok(HandleResponse::default())
}

pub fn verify_client(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    address: HumanAddr,
) -> Result<HandleResponse, ContractError> {
    let sender_address_raw = deps.api.canonical_address(&info.sender)?;
    let config_state = config(deps.storage).load()?;

    if sender_address_raw != config_state.kyc_verificator {
        return Err(ContractError::Unauthorized {});
    }


    let client_address_raw = deps.api.canonical_address(&address)?;
    let key = &client_address_raw.as_slice();
        
    clients(deps.storage).update(key, |record| {
        if let Some(mut record) = record {
            record.verified = True;
            Ok(record)
        } else {
            return Err(ContractError::ClientNotExist {});
        }
    })?;

    Ok(HandleResponse::default())
}

pub fn rent_car(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    car_id: HumanAddr,
    start: u64,
    end: u64
) -> Result<HandleResponse, ContractError> {
    let car_address_raw = deps.api.canonical_address(&car_id)?;
    let car = match rents_read(deps.storage).may_load(&car_address_raw.as_slice())? {
        Some(car) => Some(car),
        None => return Err(ContractError::CarNotExist {}),
    }
    .unwrap();

    let sender_address_raw = deps.api.canonical_address(&info.sender)?;
    let client_key = &sender_address_raw.as_slice();
    let mut client = match rents_read(deps.storage).may_load(&client_key.as_slice())? {
        Some(client) => Some(client),
        None => return Err(ContractError::ClientNotExist {}),
    }
    .unwrap();

    if !client.verified {
        return Err(StdError::generic_err("Client is not verified"));
    }    


    let rent_cost = car.deposit_price + car.rent_price * ((end - start) / RENT_PERIOD);
    if client.balance < rent_cost {
        return Err(StdError::generic_err("Insufficient funds"));
    }

    client.balance -= rent_cost;
    client.locked_balance += rent_cost;

    let rent = Rent {
        client_id: sender_address_raw,
        car_id: car_address_raw,
        balance: rent_cost,
        usage: TimePeriod{start, end},
        actual_start: 0
    };

    let mut config_state = config(deps.storage).load()?;
    let rent_id = config_state.rent_count + 1;
    config_state.rent_count = rent_id;
    let rent_key = rent_id.to_be_bytes();

    config(deps.storage).save(&config_state)?;
    clients(deps.storage).save(client_key, &client)?;
    rents(deps.storage).save(rent_key, &rent)?;

    let r = HandleResponse {
        messages: vec![],
        attributes: vec![
            attr("action", "rent_car"),
            attr("rent_id", &rent_id),
        ],
        data: Some(to_binary(&RentCarResponse { rent_id })?),
    };
    Ok(r)
}

pub fn start_rent(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    rent_id: u64,
    date: u64
) -> Result<HandleResponse, ContractError> {
    let key = &rent_id.to_be_bytes();
    rents(deps.storage).update(key, |record| {
        if let Some(mut record) = record {
            if api.canonical_address(&info.sender)? != record.car_id {
                return Err(ContractError::Unauthorized {});
            }
            record.actual_start = date;
            Ok(record)
        } else {
            return Err(ContractError::RentNotExist {});
        }
    })?;

    Ok(HandleResponse::default())
}

pub fn end_rent(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    rent_id: u64,
    date: u64
) -> Result<HandleResponse, ContractError> {
    let rent_key = &rent_id.to_be_bytes();
    let mut rent = match rents_read(deps.storage).may_load(rent_key)? {
        Some(rent) => Some(rent),
        None => return Err(ContractError::RentNotExist {}),
    }
    .unwrap();

    if rent.balance == 0 {
        return Err(StdError::generic_err("Rent is closed"));
    }

    let car_key = &rent.car_id.as_slice();
    let mut car = match rents_read(deps.storage).may_load(car_key)? {
        Some(car) => Some(car),
        None => return Err(ContractError::CarNotExist {}),
    }
    .unwrap();

    let mut payment = rent.balance - car.deposit_price;
    if date > rent.usage.end {
        payment += ((date - rent.usage.end) / RENT_PERIOD) * car.rent_price;
    }

    car.balance += payment;

    let client_key = &rent.client_id.to_be_bytes();
    clients(deps.storage).update(client_key, |record| {
        if let Some(mut record) = record {
            record.locked_balance -= rent.balance;
            record.balance += rent.balance - payment;
            Ok(record)
        } else {
            return Err(ContractError::ClientNotExist {});
        }
    })?;

    rent.balance = 0;

    rents(deps.storage).save(rent_key, &rent)?;
    cars(deps.storage).save(car_key, &car)?;

    Ok(HandleResponse::default())
}

pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::Balance { address } => client_balance(deps, address),
        QueryMsg::Rent { rent_id } => rent_by_id(deps, rent_id),
    }
}


fn client_balance(deps: Deps, address: HumanAddr) -> StdResult<Binary> {
    let sender_address_raw = deps.api.canonical_address(&info.sender)?;
    let key = &sender_address_raw.as_slice();
    
    let client = match clients_read(deps.storage).may_load(key)? {
        Some(client) => Some(client),
        None => return Err(ContractError::ClientNotExist {}),
    }
    .unwrap();

    let resp = ClientBalanceResponse {
        balance: client.balance,
        locked_balance: client.locked_balance,
    };

    to_binary(&resp)
}

fn rent_by_id(deps: Deps, rent_id: u64) -> StdResult<Binary> {
    let key = &rent_id.to_be_bytes();

    let rent = match rents_read(deps.storage).may_load(key)? {
        Some(rent) => Some(rent),
        None => return Err(ContractError::RentNotExist {}),
    }
    .unwrap();
    
    let resp = RentResponse {
        client: rent.client_id,
        car: rent.car_id,
        balance: rent.balance,
        usage_start: rent.usage.start,
        usage_end: rent.usage.end,
        actual_start: rent.actual_start,
    };
    to_binary(&resp)
}