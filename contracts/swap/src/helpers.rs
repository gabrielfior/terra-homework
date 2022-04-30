use cosmwasm_std::testing::{MockQuerier};
use cosmwasm_std::{
    from_binary, from_slice, to_binary, Addr, Coin, ContractResult, Decimal, 
    OwnedDeps, Querier, QuerierResult, QueryRequest, SystemError, SystemResult, Uint128,
    WasmQuery,
};
use cw20::TokenInfoResponse;
use std::collections::HashMap;
use std::str::FromStr;

use cw20::BalanceResponse as Cw20BalanceResponse;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use terra_cosmwasm::{
    ExchangeRateItem, ExchangeRatesResponse, TaxCapResponse, TaxRateResponse, TerraQuery,
    TerraQueryWrapper, TerraRoute,
};

#[derive(Clone, Default)]
pub struct TaxQuerier {
    rate: Decimal,
    caps: HashMap<String, Uint128>,
}

impl TaxQuerier {
    pub fn _new(rate: Decimal, caps: &[(&String, &Uint128)]) -> Self {
        TaxQuerier {
            rate,
            caps: _caps_to_map(caps),
        }
    }
}

pub(crate) fn _caps_to_map(caps: &[(&String, &Uint128)]) -> HashMap<String, Uint128> {
    let mut owner_map: HashMap<String, Uint128> = HashMap::new();
    for (denom, cap) in caps.iter() {
        owner_map.insert(denom.to_string(), **cap);
    }
    owner_map
}

pub struct WasmMockQuerier {
    base: MockQuerier<TerraQueryWrapper>,
    token_querier: TokenQuerier,
    tax_querier: TaxQuerier,
}

impl Querier for WasmMockQuerier {
    fn raw_query(&self, bin_request: &[u8]) -> QuerierResult {
        // MockQuerier doesn't support Custom, so we ignore it completely here
        let request: QueryRequest<TerraQueryWrapper> = match from_slice(bin_request) {
            Ok(v) => v,
            Err(e) => {
                return SystemResult::Err(SystemError::InvalidRequest {
                    error: format!("Parsing query request: {}", e),
                    request: bin_request.into(),
                })
            }
        };
        self.handle_query(&request)
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum QueryMsg {
    Balance { address: String },
    TokenInfo {},
    State {},
    RewardAssetWhitelist {},
    GetBoost { user: Addr },
}

impl WasmMockQuerier {
    pub fn handle_query(&self, request: &QueryRequest<TerraQueryWrapper>) -> QuerierResult {
        match &request {
            QueryRequest::Wasm(WasmQuery::Smart { contract_addr, msg }) => SystemResult::Ok(ContractResult::from(to_binary(&26))),
            _ => self.base.handle_query(request),
        }
    }
}

#[derive(Clone, Default)]
pub struct TokenQuerier {
    balances: HashMap<String, HashMap<String, Uint128>>,
}

impl TokenQuerier {
    pub fn new(balances: &[(&String, &[(&String, &Uint128)])]) -> Self {
        TokenQuerier {
            balances: balances_to_map(balances),
        }
    }

    pub fn get_balance(&self, token_addr: &str, addr: &str) -> Uint128 {
        let contract_balances = self.balances.get(&token_addr.to_string());
        match contract_balances {
            Some(balances) => *balances.get(&addr.to_string()).unwrap_or(&Uint128::zero()),
            None => Uint128::zero(),
        }
    }
}

pub(crate) fn balances_to_map(
    balances: &[(&String, &[(&String, &Uint128)])],
) -> HashMap<String, HashMap<String, Uint128>> {
    let mut balances_map: HashMap<String, HashMap<String, Uint128>> = HashMap::new();
    for (contract_addr, balances) in balances.iter() {
        let mut contract_balances_map: HashMap<String, Uint128> = HashMap::new();
        for (addr, balance) in balances.iter() {
            contract_balances_map.insert(addr.to_string(), **balance);
        }

        balances_map.insert(contract_addr.to_string(), contract_balances_map);
    }
    balances_map
}


#[derive(Clone, Default)]
pub struct VaultStateQuerier {
    total_bond_amount: Uint128,
}

impl VaultStateQuerier {
    pub fn new(total_bond_amount: &Uint128) -> Self {
        VaultStateQuerier {
            total_bond_amount: *total_bond_amount,
        }
    }
}

#[derive(Clone, Default)]
pub struct BoostQuerier {
    /// address to boost amount
    pub boost_map: HashMap<String, Uint128>,
}

impl BoostQuerier {
    pub fn get_boost(&self, addr: &Addr) -> Result<Uint128, String> {
        Ok(self
            .boost_map
            .get(&addr.to_string())
            .map_or(Uint128::zero(), |v| *v))
    }
}



impl WasmMockQuerier {
    pub fn new(base: MockQuerier<TerraQueryWrapper>) -> Self {
        WasmMockQuerier {
            base,
            token_querier: TokenQuerier::default(),
            tax_querier: TaxQuerier::default(),
        }
    }

    pub fn with_native_balances(&mut self, balances: &[(String, Coin)]) {
        for (addr, coin) in balances {
            self.base.update_balance(addr, vec![coin.clone()]);
        }
    }

    pub fn with_token_balances(&mut self, balances: &[(&String, &[(&String, &Uint128)])]) {
        self.token_querier = TokenQuerier::new(balances);
    }
}