use cosmwasm_schema::cw_serde;
use cosmwasm_std::{
    entry_point, to_json_binary, Coin, DepsMut, Env, MessageInfo, Response, StdResult, Uint128,
    WasmMsg,
};
use std::collections::BTreeMap;

#[cw_serde]
pub struct InstantiateMsg {}

#[cw_serde]
pub enum ExecuteMsg {
    Transfer {
        channel: String,
        receiver: String,
        amount: Uint128,
        denom: String,
        contract_address: String,
    },
}

pub type Fees = BTreeMap<String, Uint128>;

#[cw_serde]
pub enum Ucs01ExecuteMsg {
    Transfer(TransferMsg),
}

/// This is the message we accept via Receive
#[cw_serde]
pub struct TransferMsg {
    /// The local channel to send the packets on
    pub channel: String,
    /// The remote address to send to.
    pub receiver: String,
    /// How long the packet lives in seconds. If not specified, use default_timeout
    pub timeout: Option<u64>,
    /// The memo
    pub memo: String,
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
            receiver,
            channel,
            amount,
            denom,
            contract_address,
        } => execute_transfer(
            deps,
            info,
            receiver,
            amount,
            denom,
            channel,
            contract_address,
        ),
    }
}

pub fn execute_transfer(
    _deps: DepsMut,
    _info: MessageInfo,
    receiver: String,
    amount: Uint128,
    denom: String,
    channel: String,
    contract_address: String,
) -> StdResult<Response> {
    let msg = WasmMsg::Execute {
        contract_addr: contract_address.to_string(),
        msg: to_json_binary(&Ucs01ExecuteMsg::Transfer(TransferMsg {
            channel,
            receiver: receiver.clone(),
            timeout: None,
            memo: "".into(),
        }))?,

        funds: vec![Coin { denom, amount }],
    };

    Ok(Response::new()
        .add_message(msg)
        .add_attribute("action", "transfer")
        .add_attribute("recipient", receiver)
        .add_attribute("amount", amount.to_string()))
}
