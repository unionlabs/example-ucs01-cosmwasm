use cosmwasm_std::{
    entry_point, to_json_binary, Coin, DepsMut, Env, MessageInfo, Response, StdResult, WasmMsg,
};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct InstantiateMsg {}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum ExecuteMsg {
    Transfer {
        recipient: String,
        amount: u128,
        denom: String,
        contract_address: String,
    },
}

#[entry_point]
pub fn instantiate(
    _deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    _msg: InstantiateMsg,
) -> StdResult<Response> {
    Ok(Response::new().add_attribute("action", "instantiate"))
}

#[entry_point]
pub fn execute(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> StdResult<Response> {
    match msg {
        ExecuteMsg::Transfer {
            recipient,
            amount,
            denom,
            contract_address,
        } => execute_transfer(deps, info, recipient, amount, denom, contract_address),
    }
}

pub fn execute_transfer(
    deps: DepsMut,
    _info: MessageInfo,
    recipient: String,
    amount: u128,
    denom: String,
    contract_address: String,
) -> StdResult<Response> {
    let recipient_address = deps.api.addr_validate(&recipient)?;

    let msg = WasmMsg::Execute {
        contract_addr: contract_address.to_string(),
        msg: to_json_binary(&ExecuteMsg::Transfer {
            recipient: recipient_address.to_string(),
            amount,
            denom: denom.to_string(),
            contract_address: contract_address.to_string(),
        })?,
        funds: vec![Coin {
            denom: denom.into(),
            amount: amount.into(),
        }],
    };

    Ok(Response::new()
        .add_message(msg)
        .add_attribute("action", "transfer")
        .add_attribute("recipient", recipient)
        .add_attribute("amount", amount.to_string()))
}
