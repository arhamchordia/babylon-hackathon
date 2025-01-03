use crate::{
    contract::instantiate,
    msg::{InstantiateMsg, OracleQueryMsg},
};
use cosmwasm_std::{
    from_json,
    testing::{
        mock_dependencies, mock_dependencies_with_balances, mock_env, mock_info, MockApi,
        MockQuerier, MockStorage,
    },
    to_json_binary, Coin, ContractResult, Decimal, Empty, OwnedDeps, QuerierResult, SystemError,
    SystemResult, WasmQuery,
};
use ecosystem_adaptor::msg::BalanceResponse;
use interfaces::{Allocation, GetAllocationsResponse, QueryMsg as GaugeQueryMsg};

pub const OWNER: &str = "owner";
pub const USER: &str = "user";
pub const SUBDENOM: &str = "subdenom";
pub const DEPOSIT_DENOM: &str = "denom1";
pub const OTHER_DEPOSIT_DENOM: &str = "denom2";
pub const VAULT_DENOM: &str = "vault_denom";
pub const DEST1: &str = "dest1";
pub const DEST2: &str = "dest2";

fn basic_setup(
    deps: OwnedDeps<MockStorage, MockApi, MockQuerier, Empty>,
) -> OwnedDeps<MockStorage, MockApi, MockQuerier, Empty> {
    let mut deps = deps;
    let env = mock_env();
    let info = mock_info(USER, &[]);

    assert!(instantiate(
        deps.as_mut(),
        env.clone(),
        info,
        InstantiateMsg {
            owner: OWNER.to_string(),
            subdenom: SUBDENOM.to_string(),
            oracle: "oracle".to_string(),
            gauge: "gauge".to_string(),
        },
    )
    .is_ok());

    deps
}

pub fn setup() -> OwnedDeps<MockStorage, MockApi, MockQuerier, Empty> {
    let deps = mock_dependencies();
    basic_setup(deps)
}

pub fn setup_with_balances(
    balances: &[(&str, &[Coin])],
) -> OwnedDeps<MockStorage, MockApi, MockQuerier, Empty> {
    let deps = mock_dependencies_with_balances(balances);
    basic_setup(deps)
}

pub fn mock_wasm_querier(
    oracle: String,
    deposit_denom_price: Decimal,
    other_deposit_denom_price: Decimal,
) -> Box<impl Fn(&WasmQuery) -> QuerierResult> {
    Box::from(move |request: &WasmQuery| -> QuerierResult {
        match request {
            WasmQuery::Smart { contract_addr, msg } => {
                if contract_addr == &oracle {
                    let msg: OracleQueryMsg = from_json(msg).unwrap();
                    match msg {
                        OracleQueryMsg::Price { denom } => {
                            let response = match denom.as_str() {
                                DEPOSIT_DENOM => deposit_denom_price,
                                OTHER_DEPOSIT_DENOM => other_deposit_denom_price,
                                _ => Decimal::percent(10),
                            };
                            return SystemResult::Ok(ContractResult::Ok(
                                to_json_binary(&response).unwrap(),
                            ));
                        }
                    }
                }
                SystemResult::Err(SystemError::NoSuchContract {
                    addr: contract_addr.clone(),
                })
            }
            _ => SystemResult::Err(SystemError::Unknown {}),
        }
    })
}

#[allow(clippy::too_many_arguments)]
pub fn mock_wasm_querier_with_gauge(
    oracle: String,
    gauge: String,
    destination1: String,
    dest1_ratio: Decimal,
    dest1_balance: Vec<Coin>,
    destination2: String,
    dest2_ratio: Decimal,
    dest2_balance: Vec<Coin>,
    deposit_denom_price: Decimal,
    other_deposit_denom_price: Decimal,
) -> Box<impl Fn(&WasmQuery) -> QuerierResult> {
    Box::from(move |request: &WasmQuery| -> QuerierResult {
        match request {
            WasmQuery::Smart { contract_addr, msg } => {
                if contract_addr == &oracle {
                    let msg: OracleQueryMsg = from_json(msg).unwrap();
                    match msg {
                        OracleQueryMsg::Price { denom } => {
                            let response = match denom.as_str() {
                                DEPOSIT_DENOM => deposit_denom_price,
                                OTHER_DEPOSIT_DENOM => other_deposit_denom_price,
                                _ => Decimal::percent(10),
                            };
                            return SystemResult::Ok(ContractResult::Ok(
                                to_json_binary(&response).unwrap(),
                            ));
                        }
                    }
                }
                if contract_addr == &gauge {
                    let msg: GaugeQueryMsg = from_json(msg).unwrap();
                    match msg {
                        GaugeQueryMsg::GetAllocations {} => {
                            let response = GetAllocationsResponse {
                                allocations: vec![
                                    Allocation {
                                        destination_id: DEST1.to_string(),
                                        amount: dest1_ratio,
                                    },
                                    Allocation {
                                        destination_id: DEST2.to_string(),
                                        amount: dest2_ratio,
                                    },
                                ],
                            };
                            return SystemResult::Ok(ContractResult::Ok(
                                to_json_binary(&response).unwrap(),
                            ));
                        }
                        _ => unimplemented!(),
                    }
                }
                if contract_addr == &destination1 {
                    let response = BalanceResponse {
                        balance: dest1_balance.clone(),
                    };
                    return SystemResult::Ok(ContractResult::Ok(
                        to_json_binary(&response).unwrap(),
                    ));
                }
                if contract_addr == &destination2 {
                    let response = BalanceResponse {
                        balance: dest2_balance.clone(),
                    };
                    return SystemResult::Ok(ContractResult::Ok(
                        to_json_binary(&response).unwrap(),
                    ));
                }
                SystemResult::Err(SystemError::NoSuchContract {
                    addr: contract_addr.clone(),
                })
            }
            _ => SystemResult::Err(SystemError::Unknown {}),
        }
    })
}
