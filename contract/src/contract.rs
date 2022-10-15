#[cfg(not(feature = "library"))]
// The Essentials
use cosmwasm_std::entry_point;
use cosmwasm_std::{
    from_binary, to_binary, Addr, Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult,
};
use cw2::set_contract_version;

// The Commons
use crate::error::ContractError;
use crate::msg::*;
use crate::state::*;
use std::str;

// The Personals
use cw20::{
    Balance, Cw20CoinVerified, Cw20ReceiveMsg, BalanceResponse};

const CONTRACT_NAME: &str = "crates.io:cw20_receiver_minimal";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

////////////////////////////////////////////////////////////////////////////////////////

//////~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
///////////~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
////////////// Instantiate
///////////~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    _msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;

    let admin = info.sender.to_string();

    CONFIG.save(
        deps.storage,
        &Config {
            admin: deps.api.addr_validate(&admin)?,
        },
    )?;

    Ok(Response::new()
        .add_attribute("method", "instantiate")
        .add_attribute("admin", admin))
}

////~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
//// Execute
////
//// This contract's Execute endpoint will be called directly by the
//// Cw20 contract itself <not by the user>
////
//// To get the Cw20 contract to do this, the user will need to call 
//// the "Send{contract, amount, msg}" execute on the Cw20 contract,
//// -> Where "contract" is the Address of this contract
//// -> Where "amount" is the amount of Cw20 tokens to send to this contract
//// -> Where "msg" is <in binary> is the ReceiveMsg variant on line 102
////~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    match msg {
        // cw20 receive wrapper
        ExecuteMsg::Receive(receive_msg) => execute_receive(deps, env, info, receive_msg),
    }
}

// "Filter" for cw20 tokens
pub fn execute_receive(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    wrapper: Cw20ReceiveMsg,
) -> Result<Response, ContractError> {
    // Message included in Send{contract, amount, **msg**} execute on the cw20 contract
    let msg: ReceiveMsg = from_binary(&wrapper.msg)?;

    // Wallet that executed the "Send" on the cw20 contract
    let user_wallet = deps.api.addr_validate(&wrapper.sender)?;

    // Constructing cw20 balance
    let balance = Balance::Cw20(Cw20CoinVerified {
        // cw20 contract this message was sent from
        address: info.sender.clone(),
        // Send{contract, **amount**, msg}
        amount: wrapper.amount,
    });

    // If you wanted to add a <perhaps unnecessary> check that the user has enough tokens
    // you could query the sending contract to get user's balance & verify it's >= wrapper.amount
    //let bal_res: BalanceResponse = deps
    //    .querier
    //    .query_wasm_smart(
    //        &info.sender, 
    //        &cw20::Cw20QueryMsg::Balance {address: wrapper.sender},
    //    )?;
    
    // verify it's >= wrapper.amount
    //if bal_res.balance <= wrapper.amount {
    //    return Err(ContractError::ToDo {});
    //};


    match msg {
        // Message included in the "Send{contract, amount, **msg**}" call on the cw20 contract,
        // Sent to this contract from the cw20 contract
        ReceiveMsg::AnExecuteMsg {} => {
            execute_do_something(deps, &user_wallet, &info.sender, balance)
        }
    }
}

pub fn execute_do_something(
    _deps: DepsMut,
    _user_wallet: &Addr,
    _cw20_contract_addr: &Addr,
    _balance: Balance,
) -> Result<Response, ContractError> {

    // insert your execution logic here

    Ok(Response::default())
}

//////~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
///////////~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
////////////// Query
///////////~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::GetAdmin {} => to_binary(&get_admin(deps)?),
    }
}

pub fn get_admin(deps: Deps) -> StdResult<Binary> {
    let config = CONFIG.load(deps.storage)?;

    let admin = config.admin.to_string();

    to_binary(&AdminResponse { admin })
}

//////~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
///////////~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
////////////// Tests
///////////~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

#[cfg(test)]
mod tests {

    #[test]
    fn test1() {
        let in_prod = true;
        assert_eq!(in_prod, true);
    }

}
