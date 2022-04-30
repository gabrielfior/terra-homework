use cosmwasm_std::Addr;
use cw20::BalanceResponse;
use shared::oracle::PriceResponse;
use cosmwasm_std::{QueryRequest, WasmMsg};
use cosmwasm_std::{
    Binary, Coin, Deps, DepsMut, Empty, entry_point, Env, MessageInfo, Response, StdError, coin, StdResult, to_binary,
};
#[cfg(not(feature = "library"))]
use cosmwasm_std::{BankMsg, CosmosMsg, WasmQuery};
use cosmwasm_std::OwnedDeps;
use cosmwasm_std::testing::MockApi;
use cosmwasm_std::testing::MockQuerier;
use cosmwasm_std::testing::MockStorage;
use cosmwasm_std::Uint128;
//use cw20::BalanceResponse;
use cw2::set_contract_version;

use crate::error::ContractError;
use crate::helpers::WasmMockQuerier;
use crate::msg::{ExecuteMsg, InstantiateMsg, QueryMsg};
use crate::state::State;
use crate::state::STATE;
use cw20::{BalanceResponse as cw20_BalanceResponse, Cw20ExecuteMsg, Cw20QueryMsg};

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

    STATE.save(deps.storage, &State {
        owner: _info.sender.clone(),
        token_address: _msg.token_address,
        oracle_address: _msg.oracle_address,
    })?;

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
        ExecuteMsg::Buy {} => try_buy(_deps, _env, _info, _msg),
        ExecuteMsg::Withdraw { amount } => try_withdraw(_deps, _info, _env, amount),
    }
}

pub fn try_buy(deps: DepsMut, env: Env, info: MessageInfo, _msg: ExecuteMsg) -> Result<Response, ContractError> {
    let price_in_luna = get_price(deps.as_ref())?.price as u128;
  
    if info.funds.len() == 0 {
        return Err(ContractError::CoinMismatch {})
    }
  
    let luna_received: Uint128 = info
      .funds
      .iter()
      .find(|c| c.denom == "uluna")
      .map(|c| Uint128::from(c.amount))
      .unwrap_or_else(Uint128::zero);
  
    let coins_to_be_sent = luna_received.u128() / price_in_luna;
    let coins_in_contract = get_balance_of_cw20(deps.as_ref(), env.contract.address)?.balance.u128();
  
    if coins_in_contract < coins_to_be_sent { return Err(ContractError::InsufficientCoinsInContract {}) }
  
    let token_addr = STATE.load(deps.storage)?.token_address;
    let msg_execute = Cw20ExecuteMsg::Transfer {
        recipient: info.sender.to_string(),
        amount: Uint128::from(coins_to_be_sent),
    };
  
    Ok(Response::new().add_attributes(
      vec![
          ("price", price_in_luna.to_string()),
          ("luna_received", luna_received.to_string()),
          ("coins_sent", coins_to_be_sent.to_string()),
        ]
      ).add_message(CosmosMsg::Wasm(WasmMsg::Execute {
          contract_addr: token_addr.to_string(),
          msg: to_binary(&msg_execute)?,
          funds: vec![],
      }))
    )
  }

fn get_price(deps: Deps) -> Result<PriceResponse, ContractError> {

    let oracle_address = STATE.load(deps.storage)?.oracle_address;

    let price: PriceResponse = deps.querier.query_wasm_smart(
        oracle_address, 
        &QueryMsg::QueryPrice {})?;
    Ok(price)

}

fn get_balance_of_cw20(deps: Deps, address: Addr) -> Result<BalanceResponse, ContractError> {
    let token_address = STATE.load(deps.storage)?.token_address;
    let balance_response: BalanceResponse = deps.querier.query_wasm_smart(
        token_address,
        &QueryMsg::Balance { address: address }
    )?;
    Ok(balance_response)
  }

pub fn try_withdraw(
    deps: DepsMut,
    info: MessageInfo,
    env: Env,
    amount: i32,
) -> Result<Response, ContractError> {
    let state = STATE.load(deps.storage)?;

    if state.owner.to_string() != info.sender.clone().into_string() {
        return Err(ContractError::Unauthorized {});
    }

    let luna_balance = deps.querier.query_balance(env.contract.address,
        String::from("uluna"))?;
    
    
    let msg = CosmosMsg::Bank(BankMsg::Send {
        to_address: info.sender.clone().to_string(),
        amount: vec![coin(amount as u128, "uluna".to_string())],
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

pub const MOCK_TOKEN_ADDRESS: &str = "cosmos2contract";
pub const MOCK_ORACLE_ADDR: &str = "oracleContract";


#[cfg(test)]
mod tests {
    use cosmwasm_std::{coins};
    use cosmwasm_std::Addr;
    use cosmwasm_std::testing::{mock_env, mock_info, MOCK_CONTRACT_ADDR};
    use shared::mock_querier::{mock_dependencies};

    use super::*;

    const TOKEN: &str = "hyp0000";
    const ORACLE: &str = "oracle000";

    #[test]
    fn try_buy() {
      let mut deps = mock_dependencies(&coins(1000, TOKEN));
      deps.querier.with_oracle_price(10);
      deps.querier.with_token_balances(&[(
        &TOKEN.to_string(),
        &[(
            &MOCK_CONTRACT_ADDR.to_string(),
            &Uint128::new(1_000_000_000_000 as u128),
        )],
      )]);

      let msg = InstantiateMsg { token_address: Addr::unchecked(TOKEN), oracle_address: Addr::unchecked("oracle000") };
      let info = mock_info("creator", &coins(1_000_000, "uluna"));
      let _res = instantiate(deps.as_mut(), mock_env(), info, msg).unwrap();

      let msg = ExecuteMsg::Buy {};
      let info = mock_info("buyer", &coins(1_000, "uluna"));
      let res = execute(deps.as_mut(), mock_env(), info, msg).unwrap();
      assert_eq!("100", res.attributes[2].value);
    }

}
