use cosmwasm_std::Uint128;
use cw20_base::allowances::execute_transfer_from;
use cw20::BalanceResponse;
use crate::state::ADMIN;
use cosmwasm_std::{QueryRequest};
use cosmwasm_std::{
    entry_point, to_binary, Binary, Coin, Deps, DepsMut, Empty, Env, MessageInfo, Response,
    StdError, StdResult,
};
#[cfg(not(feature = "library"))]
use cosmwasm_std::{BankMsg, CosmosMsg, WasmQuery};

use cw2::set_contract_version;

use crate::error::ContractError;
use crate::msg::{ExecuteMsg, InstantiateMsg, QueryMsg};

// version info for migration info
const CONTRACT_NAME: &str = "crates.io:swap";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    _msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;

    ADMIN.set(deps, Some(_info.sender))?;
    Ok(Response::new().add_attribute("method", "instantiate"))
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    _deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    _msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    match _msg {
        ExecuteMsg::Buy {} => try_buy(_deps, _env, _info),
        ExecuteMsg::Withdraw { amount } => try_withdraw(_deps, _env, _info, amount),
    }
}

pub fn try_buy(deps: DepsMut, env: Env, info: MessageInfo) -> Result<Response, ContractError> {
    // queries price from other smart contract
    let oracle_contract = "terra15secglerg8y5setamsws3qnu4fv2ns7200za5j";
    let price: u64 = deps.querier.query(&QueryRequest::Wasm(WasmQuery::Smart {
        contract_addr: String::from(oracle_contract),
        msg: to_binary(&QueryMsg::QueryPrice {})?,
    }))?;
    // computes how many lemons to send buyer
    let mut num_lunas = Uint128::zero();
    for coin in info.funds.iter() {
        if coin.denom == "uluna" {
            num_lunas = coin.amount;
        }
    }

    let num_lemons_to_transfer = match num_lunas.checked_div(Uint128::from(price)) {
        Ok(n) => n,
        Err(_) => return Err(ContractError::NotImplemented {}),
    };
    
    // fails if contract does not own enough lemons in the cw20 contract
    let balance_response: BalanceResponse = deps.querier.query(&QueryRequest::Wasm(WasmQuery::Smart {
        contract_addr: String::from(oracle_contract),
        msg: to_binary(&cw20::Cw20QueryMsg::Balance {
            address: oracle_contract.to_string(),
        })?,
    }))?;

    if Uint128::from(balance_response.balance) < num_lemons_to_transfer {
        return Err(ContractError::InvalidQuantity {});
    }
    
    let b = env.contract.clone();

    // send lemons to buyer
    let res = execute_transfer_from(deps, env, info, 
        b.address.to_string(),
        oracle_contract.to_string(), 
        num_lemons_to_transfer);
    
    if res.is_err() {
        return Err(ContractError::InvalidQuantity {});
    }

    Ok(Response::new().add_attribute("method", "try_buy"))
}

pub fn try_withdraw(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    amount: i32,
) -> Result<Response, ContractError> {
    if !ADMIN.is_admin(deps.as_ref(), &info.sender)? {
        return Err(ContractError::Unauthorized {});
    }
    // this method withdraws the entire balance from smart contract
    // you might want to restrict how much UST can be withdrawn
    let balance = deps
        .querier
        .query_balance(env.contract.address.to_string(), "uluna".to_string());
    //let balance = query_balance(&deps.querier., env.contract.address.clone(), )?;
    let balance_uint = match balance {
        Ok(coin) => coin.amount,
        Err(_) => return Err(ContractError::Unauthorized {}),
    };
    let admin = match ADMIN.query_admin(deps.as_ref())?.admin {
        Some(admin_str) => admin_str,
        None => return Err(ContractError::NotImplemented {}),
    };

    let msg = CosmosMsg::Bank(BankMsg::Send {
        to_address: admin,
        amount: vec![Coin {
            denom: "uluna".to_string(),
            amount: balance_uint, // ToDo - Restrict amount to amount
        }],
    });

    Ok(Response::new().add_message(msg))
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn migrate(_deps: DepsMut, _env: Env, _msg: Empty) -> StdResult<Response> {
    // TODO
    Ok(Response::default())
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(_deps: Deps, _env: Env, _msg: QueryMsg) -> StdResult<Binary> {
    // TODO
    Err(StdError::generic_err("Not implemented"))
}

#[cfg(test)]
mod tests {
    #[test]
    fn proper_initialization() {

        //TODO
    }
}
