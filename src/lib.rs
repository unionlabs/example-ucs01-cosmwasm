use cosmwasm_std::{
    entry_point, to_json_binary, Coin, DepsMut, Env, MessageInfo, Response, StdResult,
    WasmMsg,
};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct InstantiateMsg {}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum ExecuteMsg {
    Transfer { recipient: String, amount: u128 },
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
        ExecuteMsg::Transfer { recipient, amount } => {
            execute_transfer(deps, info, recipient, amount)
        }
    }
}

pub fn execute_transfer(
    deps: DepsMut,
    info: MessageInfo,
    recipient: String,
    amount: u128,
) -> StdResult<Response> {
    let recipient_addr = deps.api.addr_validate(&recipient)?;

    let msg = WasmMsg::Execute {
        contract_addr: "union1m87a5scxnnk83wfwapxlufzm58qe2v65985exff70z95a2yr86yq7hl08h"
            .to_string(),
        msg: to_json_binary(&ExecuteMsg::Transfer {
            recipient: recipient_addr.to_string(),
            amount,
        })?,
        funds: vec![Coin {
            denom: "uusdc".to_string(),
            amount: amount.into(),
        }],
    };

    Ok(Response::new()
        .add_message(msg)
        .add_attribute("action", "transfer")
        .add_attribute("recipient", recipient)
        .add_attribute("amount", amount.to_string()))
}
